use crate::builder::Buildable;
use crate::document::Document;
use crate::helper::build_write::build_logical_write;
use crate::keyword::LogicalOperator;
use ruby_prism::ClassVariableAndWriteNode;

pub fn build_node(node: Option<&ClassVariableAndWriteNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_logical_write(name.build(), value.build(), LogicalOperator::And)
}
