use crate::builder::prism::helper::build_blank_lines::LineBreakIndex;
use crate::comments::CommentStore;
use ruby_prism::Node;

/// build context during building process
pub struct BuildContext<'sh> {
    pub source: &'sh [u8],
    pub root: &'sh Node<'sh>,
    pub built_end: usize,
    pub line_break_index: LineBreakIndex,
    pub comment_store: CommentStore<'sh>,
    pub max_blank_lines: usize,
    pub hash_label_style: bool,
    pub percent_literal: bool,
}
