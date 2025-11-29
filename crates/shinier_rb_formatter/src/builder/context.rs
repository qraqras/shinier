use crate::builder::prism::build_blank_lines::LineBreakIndex;
use crate::builder::prism::comments::CommentStore;
use std::collections::HashSet;

/// Build context during building process.
pub struct BuildContext<'sh> {
    pub previous_start_offset: usize,
    pub line_break_index: LineBreakIndex,
    pub comment_store: &'sh mut CommentStore<'sh>,
    pub processed_locations: HashSet<(usize, usize)>,
    pub max_blank_lines: usize,
    pub hash_label_style: bool,
    pub percent_literal: bool,
}
