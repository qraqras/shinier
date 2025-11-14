use crate::builder::builder::{array, break_parent, hardline, line_suffix, string};
use crate::builder::context::BuildContext;
use crate::builder::prism::comments::CommentStore;
use crate::document::Document;
use ruby_prism::Comment;
use ruby_prism::CommentType;
use ruby_prism::Location;
use ruby_prism::Node;

pub fn leading_comment_n(
    node: &Node,
    max: usize,
    comment_store: &CommentStore,
    context: &mut BuildContext,
) -> Document {
    leading_comments(
        node.location().start_offset(),
        node.location().end_offset(),
        max,
        comment_store,
        context,
    )
}

pub fn leading_comment_l(
    location: &Location,
    max: usize,
    comment_store: &CommentStore,
    context: &mut BuildContext,
) -> Document {
    leading_comments(
        location.start_offset(),
        location.end_offset(),
        max,
        comment_store,
        context,
    )
}

pub fn trailing_comment_n(
    node: &Node,
    comment_store: &CommentStore,
    context: &mut BuildContext,
) -> Document {
    trailing_comments(
        node.location().start_offset(),
        node.location().end_offset(),
        comment_store,
        context,
    )
}

pub fn trailing_comment_l(
    location: &Location,
    comment_store: &CommentStore,
    context: &mut BuildContext,
) -> Document {
    trailing_comments(
        location.start_offset(),
        location.end_offset(),
        comment_store,
        context,
    )
}

fn leading_comments(
    start_offset: usize,
    end_offset: usize,
    max: usize,
    comment_store: &CommentStore,
    context: &mut BuildContext,
) -> Document {
    let mut documents = Vec::new();
    if let Some(comment_placement) = comment_store.by_target.get(&(start_offset, end_offset)) {
        for offsets in &comment_placement.leading {
            if let Some(comment) = comment_store.by_location.get(offsets) {
                let blank_line_count = context
                    .line_break_index
                    .count_leading_blank_lines(offsets.0)
                    .min(max);
                for _ in 0..=blank_line_count {
                    documents.push(hardline());
                }
                documents.push(build_comment(comment));
            }
        }
    }
    array(&documents)
}

fn trailing_comments(
    start_offset: usize,
    end_offset: usize,
    comment_store: &CommentStore,
    _context: &mut BuildContext,
) -> Document {
    let mut documents = Vec::new();
    if let Some(comment_placement) = comment_store.by_target.get(&(start_offset, end_offset)) {
        for offsets in &comment_placement.trailing {
            if let Some(comment) = comment_store.by_location.get(offsets) {
                documents.push(line_suffix(build_comment(comment)));
            }
        }
    }
    if !documents.is_empty() {
        documents.push(break_parent());
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
