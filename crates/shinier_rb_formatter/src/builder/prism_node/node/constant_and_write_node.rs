use crate::builder::Buildable;
use crate::doc::Doc;
use crate::helper::build_write_pattern::build_logical_write_pattern;
use crate::keyword::LogicalOperator;
use ruby_prism::ConstantAndWriteNode;

pub fn build_node(node: Option<&ConstantAndWriteNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_logical_write_pattern(name.build(), value.build(), LogicalOperator::And)
}
