use crate::builder::builder::array;
use crate::builder::builder::break_parent;
use crate::builder::builder::hardline;
use crate::builder::builder::line_suffix;
use crate::builder::builder::space;
use crate::builder::builder::string;
use crate::builder::context::BuildContext;
use crate::document::Document;
use ruby_prism::Comment;
use ruby_prism::CommentType;

/// Builds comments as leading comments for a given set of comments.
pub fn build_comments_as_leading(comments: Option<Vec<Comment>>, context: &mut BuildContext) -> Option<Document> {
    match comments {
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
                documents.push(build_comment(&comment));
            }
            documents.push(hardline());
            documents.push(break_parent());
            Some(array(&documents))
        }
        None => None,
    }
}

/// Builds comments as trailing comments for a given set of comments.
pub fn build_comments_as_trailing(comments: Option<Vec<Comment>>, _context: &mut BuildContext) -> Option<Document> {
    match comments {
        Some(comments) => {
            let mut documents = Vec::new();
            for comment in comments {
                documents.push(line_suffix(array(&[space(), build_comment(&comment)])));
            }
            documents.push(break_parent());
            Some(array(&documents))
        }
        None => None,
    }
}

/// Builds comments as dangling comments for a given set of comments.
pub fn build_comments_as_dangling(comments: Option<Vec<Comment>>, context: &mut BuildContext) -> Option<Document> {
    match comments {
        Some(comments) => {
            let mut documents = Vec::new();
            for comment in comments {
                documents.push(hardline());
                // hardlines for blank lines
                let blank_line_count = context
                    .line_break_index
                    .count_leading_blank_lines(comment.location().start_offset())
                    .min(context.max_blank_lines);
                for _ in 0..blank_line_count {
                    documents.push(hardline());
                }
                documents.push(build_comment(&comment));
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
