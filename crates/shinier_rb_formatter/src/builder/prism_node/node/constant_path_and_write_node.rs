use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::document::Document;
use ruby_prism::Comments;
use crate::helper::build_write::build_logical_write;
use crate::keyword::LogicalOperator;
use ruby_prism::ConstantPathAndWriteNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(node: Option<&ConstantPathAndWriteNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    build_logical_write(
        target.as_node().build(comments),
        value.build(comments),
        LogicalOperator::And,
    )
}
