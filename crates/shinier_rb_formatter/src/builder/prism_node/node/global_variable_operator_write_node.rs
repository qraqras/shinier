use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::document::Document;
use ruby_prism::Comments;
use crate::helper::build_write::build_operator_write;
use ruby_prism::GlobalVariableOperatorWriteNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&GlobalVariableOperatorWriteNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    let binary_operator = node.binary_operator();
    build_operator_write(name.build(comments), value.build(comments), binary_operator.build(comments))
}
