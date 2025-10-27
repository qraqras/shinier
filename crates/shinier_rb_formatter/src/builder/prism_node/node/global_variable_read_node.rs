use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::GlobalVariableReadNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(node: Option<&GlobalVariableReadNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    node.name().build(comments)
}
