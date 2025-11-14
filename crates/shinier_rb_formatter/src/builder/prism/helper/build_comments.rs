use crate::builder::builder::{array, break_parent, hardline, line_suffix, space, string};
use crate::builder::context::BuildContext;
use crate::document::Document;
use ruby_prism::Comment;
use ruby_prism::CommentType;
use ruby_prism::Location;
use ruby_prism::Node;

/// Builds leading comments for a given node.
pub fn leading_comments_n(node: &Node, context: &mut BuildContext) -> Option<Document> {
    base_leading_comments(
        node.location().start_offset(),
        node.location().end_offset(),
        context,
    )
}

/// Builds leading comments for a given location.
pub fn leading_comments_l(location: &Location, context: &mut BuildContext) -> Option<Document> {
    base_leading_comments(location.start_offset(), location.end_offset(), context)
}

/// Builds trailing comments for a given node.
pub fn trailing_comments_n(node: &Node, context: &mut BuildContext) -> Option<Document> {
    base_trailing_comments(
        node.location().start_offset(),
        node.location().end_offset(),
        context,
    )
}

/// Builds trailing comments for a given location.
pub fn trailing_comments_l(location: &Location, context: &mut BuildContext) -> Option<Document> {
    base_trailing_comments(location.start_offset(), location.end_offset(), context)
}

/// Builds leading comments for a given start and end offset.
fn base_leading_comments(
    start_offset: usize,
    end_offset: usize,
    context: &mut BuildContext,
) -> Option<Document> {
    let mut documents = Vec::new();
    let comment_store = &context.comment_store;
    if let Some(comment_placement) = comment_store.by_target.get(&(start_offset, end_offset)) {
        for offsets in &comment_placement.leading {
            if let Some(comment) = comment_store.by_location.get(offsets) {
                // hardline between comments
                if !documents.is_empty() {
                    documents.push(hardline());
                }
                // hardlines for blank lines
                let blank_line_count = context
                    .line_break_index
                    .count_leading_blank_lines(offsets.0)
                    .min(context.max_blank_lines);
                for _ in 0..blank_line_count {
                    documents.push(hardline());
                }
                documents.push(build_comment(comment));
            }
        }
    }
    match documents.is_empty() {
        true => None,
        false => {
            documents.push(hardline());
            documents.push(break_parent());
            Some(array(&documents))
        }
    }
}

/// Builds trailing comments for a given start and end offset.
fn base_trailing_comments(
    start_offset: usize,
    end_offset: usize,
    context: &mut BuildContext,
) -> Option<Document> {
    let mut documents = Vec::new();
    let comment_store = &context.comment_store;
    if let Some(comment_placement) = comment_store.by_target.get(&(start_offset, end_offset)) {
        for offsets in &comment_placement.trailing {
            if let Some(comment) = comment_store.by_location.get(offsets) {
                documents.push(line_suffix(array(&[space(), build_comment(comment)])));
            }
        }
    }
    match documents.is_empty() {
        true => None,
        false => {
            documents.push(break_parent());
            Some(array(&documents))
        }
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
