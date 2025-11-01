use ruby_prism::Comments;
use ruby_prism::Node;
use std::iter::Peekable;

/// Build context for formatting.
pub struct BuildContext<'sh> {
    pub source: &'sh [u8],
    pub root: Node<'sh>,
    pub built_end: usize,
    pub comments: &'sh mut Peekable<Comments<'sh>>,
    pub max_leading_line_breaks: usize,
}
