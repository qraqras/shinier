use crate::builder::Buildable;
use crate::doc::Doc;
use crate::helper::build_write_pattern::build_logical_write_pattern;
use crate::keyword::LogicalOperator;
use ruby_prism::ConstantPathAndWriteNode;

pub fn build_node(node: Option<&ConstantPathAndWriteNode>) -> Doc {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    build_logical_write_pattern(
        target.as_node().build(),
        value.build(),
        LogicalOperator::And,
    )
}
