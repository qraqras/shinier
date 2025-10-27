use crate::document::Document;
use crate::helper::build_write::build_logical_write;
use crate::keyword::LogicalOperator;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::GlobalVariableOrWriteNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&GlobalVariableOrWriteNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    build_logical_write(
        name.build(comments),
        value.build(comments),
        LogicalOperator::Or,
    )
}
