use crate::builder::Buildable;
use crate::builder::builder::{array, line, space, string};
use crate::document::Document;
use crate::keyword::LOGICAL_AND;
use ruby_prism::AndNode;

pub fn build_node(node: Option<&AndNode>) -> Document {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    array(&[
        left.build(),
        space(),
        string(LOGICAL_AND),
        line(),
        right.build(),
    ])
}
