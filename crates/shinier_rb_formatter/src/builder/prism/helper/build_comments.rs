use crate::BuildContext;
use crate::builder::builder::{array, break_parent, hardline, line_suffix, string};
use crate::builder::prism::leading_line_breaks;
use crate::document::Document;
use ruby_prism::Node;

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
                    let text = std::str::from_utf8(comment.text()).unwrap();
                    documents.push(string(text));
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
pub fn owning_comments(node: &Node, context: &mut BuildContext) -> Document {
    let mut documents = Vec::new();
    loop {
        match context.comments.peek() {
            Some(comment) => {
                let comment_start_offset = comment.location().start_offset();
                if comment_start_offset >= node.location().start_offset()
                    && comment_start_offset < node.location().end_offset()
                {
                    documents.push(leading_line_breaks(context, comment_start_offset, 1usize));
                    let comment = context.comments.next().unwrap();
                    let text = std::str::from_utf8(comment.text()).unwrap();
                    documents.push(string(text));
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
