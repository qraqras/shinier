use crate::builder::builder::*;
use crate::builder::context::BuildContext;
use crate::builder::prism::comments::CommentPosition;
use crate::builder::prism::comments::CommentWrapper;
use crate::document::Document;
use ruby_prism::Comment;
use ruby_prism::CommentType;

/// Builds comments as leading comments for a given set of comments.
pub fn build_comments_as_leading(
    comment_wrappers: Option<Vec<CommentWrapper>>,
    ctx: &mut BuildContext,
) -> Option<Document> {
    match comment_wrappers {
        Some(comment_wrappers) => {
            let mut documents = Vec::new();
            for comment_wrapper in comment_wrappers {
                let comment = comment_wrapper.comment;
                // hardline between comments
                if !documents.is_empty() {
                    documents.push(hardline());
                }
                // hardlines for blank lines
                let blank_line_count = ctx
                    .line_break_index
                    .count_leading_blank_lines(comment.location().start_offset())
                    .min(ctx.max_blank_lines);
                for _ in 0..blank_line_count {
                    documents.push(hardline());
                }
                documents.push(_build_comment(&comment));
            }
            documents.push(hardline());
            documents.push(break_parent());
            array(&documents)
        }
        None => None,
    }
}

/// Builds comments as trailing comments for a given set of comments.
pub fn build_comments_as_trailing(
    comment_wrappers: Option<Vec<CommentWrapper>>,
    _ctx: &mut BuildContext,
) -> Option<Document> {
    match comment_wrappers {
        Some(comment_wrappers) if !comment_wrappers.is_empty() => {
            let mut documents = Vec::new();
            for comment_wrapper in comment_wrappers {
                let comment = comment_wrapper.comment;
                let position = comment_wrapper.position;
                match position {
                    CommentPosition::EndOfLine => {
                        documents.push(line_suffix(array(&[space(), _build_comment(&comment)])));
                        documents.push(break_parent());
                    }
                    CommentPosition::OwnLine => {
                        documents.push(hardline());
                        documents.push(_build_comment(&comment));
                    }
                }
            }
            array(&documents)
        }
        Some(_comments) => None,
        None => None,
    }
}

/// Builds comments as dangling comments for a given set of comments.
pub fn build_comments_as_dangling(
    comment_wrappers: Option<Vec<CommentWrapper>>,
    ctx: &mut BuildContext,
) -> Option<Document> {
    match comment_wrappers {
        Some(comment_wrappers) if !comment_wrappers.is_empty() => {
            let mut documents = Vec::new();
            for comment_wrapper in comment_wrappers {
                let comment = comment_wrapper.comment;
                documents.push(hardline());
                // hardlines for blank lines
                let blank_line_count = ctx
                    .line_break_index
                    .count_leading_blank_lines(comment.location().start_offset())
                    .min(ctx.max_blank_lines);
                for _ in 0..blank_line_count {
                    documents.push(hardline());
                }
                documents.push(_build_comment(&comment));
            }
            indent(array(&documents))
        }
        Some(_comments) => None,
        None => None,
    }
}

/// Builds a comment into a Document.
/// If the comment is an embedded document comment (=begin ... =end),
/// it formats it as multiple lines with '#' prefixes.
fn _build_comment(comment: &Comment) -> Option<Document> {
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
