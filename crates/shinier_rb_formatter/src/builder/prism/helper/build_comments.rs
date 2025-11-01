use crate::BuildContext;
use crate::builder::builder::{array, break_parent, hardline, line_suffix, string};
use crate::builder::prism::leading_line_breaks;
use crate::document::Document;
use ruby_prism::Comment;
use ruby_prism::CommentType;
use ruby_prism::Node;
use ruby_prism::Visit;

struct TrailingVisitor {
    pub node_end_offset: usize,
    pub line_end_offset: usize,
    pub has_node_between: bool,
}

impl<'sh> Visit<'sh> for TrailingVisitor {
    fn visit_branch_node_enter(&mut self, node: Node<'sh>) {
        if self.has_node_between {
            return;
        }
        let start = node.location().start_offset();
        let end = node.location().end_offset();
        if end <= self.node_end_offset || start >= self.line_end_offset {
            return;
        }
        if self.node_end_offset < start && start < self.line_end_offset {
            self.has_node_between = true;
        }
    }

    fn visit_leaf_node_enter(&mut self, node: Node<'sh>) {
        if self.has_node_between {
            return;
        }
        let start = node.location().start_offset();
        if self.node_end_offset < start && start < self.line_end_offset {
            self.has_node_between = true;
        }
    }
}

/// Builds leading comments for a given node.
/// ```ruby
/// # leading comment
/// foo
/// ```
pub fn leading_comments(node: &Node, context: &mut BuildContext) -> Document {
    let mut documents = Vec::new();
    loop {
        match context.comments.peek() {
            Some(comment) => {
                let comment_start_offset = comment.location().start_offset();
                if comment.location().start_offset() < node.location().start_offset() {
                    documents.push(leading_line_breaks(context, comment_start_offset, 1usize));
                    let comment = context.comments.next().unwrap();
                    documents.push(build_comment(&comment));
                    documents.push(hardline());
                    context.built_end = comment.location().end_offset();
                    continue;
                }
                break;
            }
            None => break,
        }
    }
    array(&documents)
}

/// Builds owning comments for a given node.
/// ```ruby
/// if foo then
///   # owning comment
/// end
/// ```
pub fn owning_comments(node: &Node, context: &mut BuildContext) -> Option<Document> {
    let mut linebreaks = Vec::new();
    let mut documents = Vec::new();
    loop {
        match context.comments.peek() {
            Some(comment) => {
                let comment_start_offset = comment.location().start_offset();
                if comment_start_offset >= node.location().start_offset()
                    && comment_start_offset < node.location().end_offset()
                {
                    if context.built_end == comment_start_offset - 1 {
                        linebreaks.push(hardline());
                    } else {
                        documents.push(leading_line_breaks(context, comment_start_offset, 1usize));
                    }
                    let comment = context.comments.next().unwrap();
                    documents.push(build_comment(&comment));
                    documents.push(hardline());
                    context.built_end = comment.location().end_offset();
                    continue;
                }
                break;
            }
            None => break,
        }
    }
    if !documents.is_empty() {
        // remove last hardline and add break parent
        documents.pop();
        documents.push(break_parent());
        return Some(array(&documents));
    }
    None
}

/// Builds trailing comments for a given node.
/// ```ruby
/// foo # trailing comment
/// ```
pub fn trailing_comments(node: &Node, context: &mut BuildContext) -> Document {
    let mut documents = Vec::new();
    loop {
        match context.comments.peek() {
            Some(comment) => {
                let comment_start_offset = comment.location().start_offset();
                // scan to the end of the line
                let mut line_end_offset = node.location().end_offset();
                while line_end_offset < context.source.len()
                    && context.source[line_end_offset] != b'\n'
                {
                    line_end_offset += 1;
                }

                // TODO: パフォーマンス改善の余地あり
                let mut visitor = TrailingVisitor {
                    node_end_offset: node.location().end_offset(),
                    line_end_offset,
                    has_node_between: false,
                };
                visitor.visit(&context.root);
                if visitor.has_node_between {
                    break;
                }

                if comment_start_offset >= node.location().end_offset()
                    && comment_start_offset < line_end_offset
                {
                    let comment = context.comments.next().unwrap();
                    let text = std::str::from_utf8(comment.text()).unwrap();
                    documents.push(line_suffix(string(format!(" {}", text))));
                    documents.push(break_parent());
                    context.built_end = comment.location().end_offset();
                    continue;
                }
                break;
            }
            None => break,
        }
    }
    array(&documents)
}

/// Builds the rest of the comments in the source code.
/// ```ruby
/// # rest comment
/// EOF
///```
pub fn rest_comments(context: &mut BuildContext) -> Document {
    let mut documents = Vec::new();
    loop {
        match context.comments.next() {
            Some(comment) => {
                documents.push(leading_line_breaks(
                    context,
                    comment.location().start_offset(),
                    1usize,
                ));
                documents.push(build_comment(&comment));
                documents.push(hardline());
                continue;
            }
            None => break,
        }
    }
    if !documents.is_empty() {
        // remove last hardline
        documents.pop();
    }
    array(&documents)
}

/// Builds a Document for a given comment.
/// If the comment is an embedded document comment (=begin ... =end),
/// it formats it as multiple lines with '#' prefixes.
fn build_comment(comment: &Comment) -> Document {
    let text = std::str::from_utf8(comment.text()).unwrap();
    match comment.type_() {
        CommentType::EmbDocComment => {
            let mut lines: Vec<&str> = text.lines().collect();
            if let Some(first) = lines.first() {
                if first.trim_start().starts_with("=begin") {
                    lines.remove(0);
                }
            }
            if let Some(last) = lines.last() {
                if last.trim_start().starts_with("=end") {
                    lines.pop();
                }
            }
            let mut documents = Vec::new();
            for (i, line) in lines.iter().enumerate() {
                if i > 0 {
                    documents.push(hardline());
                }
                documents.push(string(format!("# {}", line)));
            }
            array(&documents)
        }
        CommentType::InlineComment => string(text),
    }
}
