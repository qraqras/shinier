use crate::builder::builder::{array, break_parent, hardline, line_suffix, space, string};
use crate::builder::context::BuildContext;
use crate::document::Document;
use ruby_prism::Comment;
use ruby_prism::CommentType;
use ruby_prism::Location;
use ruby_prism::Node;

/// Builds leading comments for a given node.
pub fn leading_comments_n(node: &Node, context: &mut BuildContext) -> Option<Document> {
    base_leading_comments(node.location().start_offset(), node.location().end_offset(), context)
}

/// Builds leading comments for a given location.
pub fn leading_comments_l(location: &Location, context: &mut BuildContext) -> Option<Document> {
    base_leading_comments(location.start_offset(), location.end_offset(), context)
}

/// Builds trailing comments for a given node.
pub fn trailing_comments_n(node: &Node, context: &mut BuildContext) -> Option<Document> {
    base_trailing_comments(node.location().start_offset(), node.location().end_offset(), context)
}

/// Builds trailing comments for a given location.
pub fn trailing_comments_l(location: &Location, context: &mut BuildContext) -> Option<Document> {
    base_trailing_comments(location.start_offset(), location.end_offset(), context)
}

/// Builds leading comments for a given start and end offset.
fn base_leading_comments(start_offset: usize, end_offset: usize, context: &mut BuildContext) -> Option<Document> {
    let leading_comments = context.comment_store.pop_leading(start_offset, end_offset);
    match leading_comments {
        Some(comments) => {
            let mut documents = Vec::new();
            for comment in comments {
                // hardline between comments
                if !documents.is_empty() {
                    documents.push(hardline());
                }
                // hardlines for blank lines
                let blank_line_count = context
                    .line_break_index
                    .count_leading_blank_lines(comment.location().start_offset())
                    .min(context.max_blank_lines);
                for _ in 0..blank_line_count {
                    documents.push(hardline());
                }
                documents.push(build_comment(comment));
            }
            documents.push(hardline());
            documents.push(break_parent());
            Some(array(&documents))
        }
        None => None,
    }
}

/// Builds indented leading comments for a given start and end offset.
/// ```ruby
/// if condition
///   # This is an indented leading comment
/// # This is a leading comment
/// else
/// ```
fn indent_leading_comments(start_offset: usize, end_offset: usize, context: &mut BuildContext) -> Option<Document> {
    // TODO: Implement indentation logic for leading comments
}

/// Builds trailing comments for a given start and end offset.
fn base_trailing_comments(start_offset: usize, end_offset: usize, context: &mut BuildContext) -> Option<Document> {
    let trailing_comments = context.comment_store.pop_trailing(start_offset, end_offset);
    match trailing_comments {
        Some(comments) => {
            let mut documents = Vec::new();
            for comment in comments {
                documents.push(line_suffix(array(&[space(), build_comment(comment)])));
            }
            documents.push(break_parent());
            Some(array(&documents))
        }
        None => None,
    }
}

/// Builds a comment into a Document.
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
