use crate::document::Document;
use crate::helper::build_write::build_operator_write;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::InstanceVariableOperatorWriteNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&InstanceVariableOperatorWriteNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    let binary_operator = node.binary_operator();
    build_operator_write(
        name.build(comments),
        value.build(comments),
        binary_operator.build(comments),
    )
}
