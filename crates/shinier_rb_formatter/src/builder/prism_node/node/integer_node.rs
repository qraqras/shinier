use crate::builder::builder::none;
use crate::document::Document;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::IntegerNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&IntegerNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    match node {
        Some(node) => {
            let location = node.location();
            location.build(comments)
        }
        None => none(),
    }
}
