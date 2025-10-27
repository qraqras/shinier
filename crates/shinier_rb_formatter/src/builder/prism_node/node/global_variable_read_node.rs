use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::GlobalVariableReadNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&GlobalVariableReadNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    node.name().build(comments)
}
