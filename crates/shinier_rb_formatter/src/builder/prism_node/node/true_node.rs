use crate::builder::builder::{none, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::TRUE;
use ruby_prism::TrueNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(node: Option<&TrueNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    match node {
        Some(_) => string(TRUE),
        None => none(),
    }
}
