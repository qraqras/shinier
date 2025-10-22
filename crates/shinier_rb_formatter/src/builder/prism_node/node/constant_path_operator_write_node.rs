use crate::builder::Buildable;
use crate::document::Document;
use crate::helper::build_write::build_operator_write;
use ruby_prism::ConstantPathOperatorWriteNode;

pub fn build_node(node: Option<&ConstantPathOperatorWriteNode>) -> Document {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    let binary_operator = node.binary_operator();
    build_operator_write(
        target.as_node().build(),
        value.build(),
        binary_operator.build(),
    )
}
