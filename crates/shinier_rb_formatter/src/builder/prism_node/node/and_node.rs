use crate::builder::Buildable;
use crate::builder::builder::{array, fill, space, string};
use crate::document::Document;
use crate::keyword::LogicalOperator;
use ruby_prism::AndNode;

pub fn build_node(node: Option<&AndNode>) -> Document {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    fill(array(&[
        left.build(),
        space(),
        string(LogicalOperator::And.as_str()),
        space(),
        right.build(),
    ]))
}
