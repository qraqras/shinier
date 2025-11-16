use crate::builder::prism::helper::build_blank_lines::LineBreakIndex;
use crate::comments::CommentStore;

/// build context during building process
pub struct BuildContext<'sh> {
    pub last_processed_start_offset: usize,
    pub line_break_index: LineBreakIndex,
    pub comment_store: CommentStore<'sh>,
    pub max_blank_lines: usize,
    pub hash_label_style: bool,
    pub percent_literal: bool,
}
