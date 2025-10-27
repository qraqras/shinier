use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::FloatNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&FloatNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let location = node.location();
    location.build(comments)
}
