use crate::builder::Buildable;
use crate::doc::Doc;
use crate::helper::build_write::build_logical_write;
use crate::keyword::LogicalOperator;
use ruby_prism::ConstantOrWriteNode;

pub fn build_node(node: Option<&ConstantOrWriteNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_logical_write(name.build(), value.build(), LogicalOperator::Or)
}
