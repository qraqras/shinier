use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use ruby_prism::*;

pub fn line_if_has_comments(start_offset: usize, end_offset: usize, ctx: &BuildContext) -> Option<Document> {
    let has_comments = ctx.comment_store.has_comments_between(start_offset, end_offset);
    match has_comments {
        true => softline(),
        false => space(),
    }
}

pub fn softline_if_has_comments(start_offset: usize, end_offset: usize, ctx: &BuildContext) -> Option<Document> {
    let has_comments = ctx.comment_store.has_comments_between(start_offset, end_offset);
    match has_comments {
        true => softline(),
        false => None,
    }
}
