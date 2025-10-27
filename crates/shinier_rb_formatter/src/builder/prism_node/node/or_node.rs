use crate::BuildPrismNode;
use crate::builder::builder::{array, indent, line, space, string};
use crate::document::Document;
use crate::keyword::LogicalOperator;
use ruby_prism::Comments;
use ruby_prism::OrNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&OrNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    array(&[
        left.build(comments),
        space(),
        string(LogicalOperator::Or.as_str()),
        indent(array(&[line(), right.build(comments)])),
    ])
}
