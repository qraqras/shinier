use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::ImplicitNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&ImplicitNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let value = node.value();
    value.build(comments)
}
