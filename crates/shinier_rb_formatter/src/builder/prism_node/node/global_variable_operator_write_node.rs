use crate::builder::Buildable;
use crate::document::Document;
use crate::helper::build_write::build_operator_write;
use ruby_prism::GlobalVariableOperatorWriteNode;

pub fn build_node(node: Option<&GlobalVariableOperatorWriteNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    let binary_operator = node.binary_operator();
    build_operator_write(name.build(), value.build(), binary_operator.build())
}
