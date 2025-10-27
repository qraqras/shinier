use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::document::Document;
use ruby_prism::Comments;
use crate::helper::build_write::build_logical_write;
use crate::keyword::LogicalOperator;
use ruby_prism::ConstantPathOrWriteNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&ConstantPathOrWriteNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    build_logical_write(target.as_node().build(comments), value.build(comments), LogicalOperator::Or)
}
