use ruby_prism::Comments;
use std::iter::Peekable;

/// Build context for formatting.
pub struct BuildContext<'sh> {
    pub source: &'sh [u8],
    pub built_end: usize,
    pub comments: &'sh mut Peekable<Comments<'sh>>,
    pub max_leading_line_breaks: usize,
}
