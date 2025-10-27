use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::ClassVariableTargetNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&ClassVariableTargetNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build(comments)
}
