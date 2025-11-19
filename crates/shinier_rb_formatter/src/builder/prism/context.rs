use crate::builder::prism::helper::build_blank_lines::LineBreakIndex;
use crate::comments::CommentStore;
use ruby_prism::Comment;
use std::collections::HashSet;

/// build context during building process
pub struct BuildContext<'sh> {
    pub last_processed_start_offset: usize,
    pub line_break_index: LineBreakIndex,
    pub comment_store: &'sh mut CommentStore<'sh>,
    pub processed_locations: HashSet<(usize, usize)>,
    pub leading_comments: Option<Vec<Comment<'sh>>>,
    pub trailing_comments: Option<Vec<Comment<'sh>>>,
    pub dangling_comments: Option<Vec<Comment<'sh>>>,
    pub remaining_comments: Option<Vec<Comment<'sh>>>,
    pub comment_indentation: bool,
    pub max_blank_lines: usize,
    pub hash_label_style: bool,
    pub percent_literal: bool,
}
