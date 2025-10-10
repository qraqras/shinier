use crate::builder::build;
use crate::doc::{Doc, line, sequence, space, text};
use ruby_prism::AndNode;

const AND_KEYWORD: &str = "and";

pub fn build_node(node: Option<&AndNode>) -> Doc {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    sequence(&[
        build(&left),
        space(),
        text(AND_KEYWORD),
        line(),
        build(&right),
    ])
}
