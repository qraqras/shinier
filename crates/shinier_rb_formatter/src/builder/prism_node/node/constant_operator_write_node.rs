use crate::builder::Buildable;
use crate::document::*;
use crate::helper::build_write::build_operator_write;
use ruby_prism::ConstantOperatorWriteNode;

pub fn build_node(node: Option<&ConstantOperatorWriteNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    let binary_operator = node.binary_operator();
    build_operator_write(name.build(), value.build(), binary_operator.build())
}
