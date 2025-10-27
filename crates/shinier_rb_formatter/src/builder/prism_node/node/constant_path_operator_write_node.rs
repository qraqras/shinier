use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::document::Document;
use ruby_prism::Comments;
use crate::helper::build_write::build_operator_write;
use ruby_prism::ConstantPathOperatorWriteNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(node: Option<&ConstantPathOperatorWriteNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    let binary_operator = node.binary_operator();
    build_operator_write(
        target.as_node().build(comments),
        value.build(comments),
        binary_operator.build(comments),
    )
}
