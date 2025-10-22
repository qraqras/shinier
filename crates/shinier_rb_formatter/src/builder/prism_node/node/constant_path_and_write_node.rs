use crate::builder::Buildable;
use crate::document::*;
use crate::helper::build_write::build_logical_write;
use crate::keyword::LogicalOperator;
use ruby_prism::ConstantPathAndWriteNode;

pub fn build_node(node: Option<&ConstantPathAndWriteNode>) -> Doc {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    build_logical_write(
        target.as_node().build(),
        value.build(),
        LogicalOperator::And,
    )
}
