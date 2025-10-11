use crate::builder::build;
use crate::doc::{Doc, line, sequence, space, text};
use crate::keyword::ALTERNATION;
use ruby_prism::AlternationPatternNode;

pub fn build_node(node: Option<&AlternationPatternNode>) -> Doc {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    sequence(&[
        build(&left),
        space(),
        text(ALTERNATION),
        line(),
        build(&right),
    ])
}
