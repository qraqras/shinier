use ruby_prism::Comment;
use ruby_prism::Comments;
use std::iter::Peekable;

pub struct BuildContext<'sh> {
    pub source: &'sh [u8],
    pub built_end: usize,
    pub comments: &'sh mut Peekable<Comments<'sh>>,
    pub inner_comment: Vec<Comment<'sh>>,
    pub leading_line_breaks: bool,
}
