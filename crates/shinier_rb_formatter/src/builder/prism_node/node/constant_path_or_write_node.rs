use crate::builder::Buildable;
use crate::doc::Doc;
use crate::helper::build_write::build_logical_write;
use crate::keyword::LogicalOperator;
use ruby_prism::ConstantPathOrWriteNode;

pub fn build_node(node: Option<&ConstantPathOrWriteNode>) -> Doc {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    build_logical_write(target.as_node().build(), value.build(), LogicalOperator::Or)
}
