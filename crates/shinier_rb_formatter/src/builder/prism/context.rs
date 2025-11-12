use crate::builder::prism::helper::build_blank_lines::LineBreakIndex;
use crate::builder::prism::helper::build_comments::CommentMetadata;
use ruby_prism::Comments;
use ruby_prism::Node;
use std::collections::HashMap;
use std::iter::Peekable;

/// build context during building process
pub struct BuildContext<'sh> {
    pub source: &'sh [u8],
    pub root: &'sh Node<'sh>,
    pub built_end: usize,
    pub line_break_index: LineBreakIndex,
    pub comments: &'sh mut Peekable<Comments<'sh>>,
    pub comment_metadata: HashMap<usize, CommentMetadata>,
    pub max_blank_lines: usize,
    pub hash_label_style: bool,
    pub percent_literal: bool,
}
