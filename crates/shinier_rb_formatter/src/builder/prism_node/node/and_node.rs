use crate::builder::Buildable;
use crate::doc::{Doc, indent, line, sequence, space, text};
use crate::keyword::LOGICAL_AND;
use ruby_prism::AndNode;

pub fn build_node(node: Option<&AndNode>) -> Doc {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    sequence(&[
        left.build(),
        space(),
        text(LOGICAL_AND),
        line(),
        indent(&[right.build()]),
    ])
}
