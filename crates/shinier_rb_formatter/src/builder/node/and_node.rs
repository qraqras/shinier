use crate::builder::build;
use crate::doc::{Doc, line, sequence, space, text};
use crate::keyword::LOGICAL_AND;
use ruby_prism::AndNode;

pub fn build_node(node: Option<&AndNode>) -> Doc {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    sequence(&[
        build(&left),
        space(),
        text(LOGICAL_AND),
        line(),
        build(&right),
    ])
}
