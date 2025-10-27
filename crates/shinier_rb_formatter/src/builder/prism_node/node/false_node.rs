use crate::builder::builder::{none, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::FALSE;
use ruby_prism::FalseNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(node: Option<&FalseNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    match node {
        Some(_) => string(FALSE),
        None => none(),
    }
}
