use crate::builder::builder::none;
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::ImplicitRestNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(_node: Option<&ImplicitRestNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    none()
}
