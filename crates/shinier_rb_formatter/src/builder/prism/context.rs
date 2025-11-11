use crate::builder::prism::helper::CommentMetadata;
use ruby_prism::Comments;
use ruby_prism::Node;
use std::collections::HashMap;
use std::iter::Peekable;

/// Build context for formatting.
pub struct BuildContext<'sh> {
    pub source: &'sh [u8],
    pub root: &'sh Node<'sh>,
    pub built_end: usize,
    pub comments: &'sh mut Peekable<Comments<'sh>>,
    pub comment_metadata: HashMap<usize, CommentMetadata>,
    pub max_blank_lines: usize,
    pub hash_label_style: bool,
    pub percent_literal: bool,
}
