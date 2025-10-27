use crate::document::Document;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::LocalVariableReadNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&LocalVariableReadNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build(comments)
}
