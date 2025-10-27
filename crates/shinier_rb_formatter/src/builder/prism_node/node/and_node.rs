use crate::builder::builder::{array, fill, space, string};
use crate::document::Document;
use crate::keyword::LogicalOperator;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::AndNode;
use ruby_prism::Comments;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&AndNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    fill(array(&[
        left.build(comments),
        space(),
        string(LogicalOperator::And.as_str()),
        space(),
        right.build(comments),
    ]))
}
