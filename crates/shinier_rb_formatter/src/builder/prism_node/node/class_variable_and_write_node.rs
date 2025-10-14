use crate::builder::Buildable;
use crate::doc::Doc;
use crate::helper::write_node::build_logical_write_node;
use crate::keyword::LogicalOperator;
use ruby_prism::ClassVariableAndWriteNode;

pub fn build_node(node: Option<&ClassVariableAndWriteNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_logical_write_node(name.build(), value.build(), LogicalOperator::And)
}
