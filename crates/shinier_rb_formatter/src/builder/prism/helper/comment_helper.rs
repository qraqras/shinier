use crate::builder::prism::helper::build_blank_lines::LineBreakIndex;
use ruby_prism::Comment;
use ruby_prism::Node;

/// Separates comments into two groups based on their position relative to the node.
///
/// Returns (dangling_comments, remaining_comments):
/// - dangling_comments: Comments at or after the node's start (inside the block)
/// - remaining_comments: Comments before the node's start (outside the block)
pub fn separate_indented_comments<'sh>(
    node: &Node,
    comments: &mut Option<Vec<Comment<'sh>>>,
    line_index: &LineBreakIndex,
) -> Option<(Option<Vec<Comment<'sh>>>, Option<Vec<Comment<'sh>>>)> {
    match comments.take() {
        Some(mut comments) => {
            let mut dangling_comments = Vec::new();
            let mut remaining_comments = Vec::new();
            let node_col = line_index.col_at_offset(node.location().start_offset());

            let mut should_dangling = true;
            for comment in comments.drain(..) {
                let comment_col = line_index.col_at_offset(comment.location().start_offset());
                should_dangling = should_dangling && node_col <= comment_col;
                if should_dangling {
                    dangling_comments.push(comment);
                } else {
                    remaining_comments.push(comment);
                }
            }
            match (dangling_comments.is_empty(), remaining_comments.is_empty()) {
                (true, true) => None,
                (false, true) => Some((Some(dangling_comments), None)),
                (true, false) => Some((None, Some(remaining_comments))),
                (false, false) => Some((Some(dangling_comments), Some(remaining_comments))),
            }
        }
        None => None,
    }
}
