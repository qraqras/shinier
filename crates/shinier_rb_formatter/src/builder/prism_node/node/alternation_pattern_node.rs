use crate::builder::Buildable;
use crate::doc::{Doc, line, sequence, space, text};
use crate::keyword::ALTERNATION;
use ruby_prism::AlternationPatternNode;

pub fn build_node(node: Option<&AlternationPatternNode>) -> Doc {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    sequence(&[
        left.build(),
        space(),
        text(ALTERNATION),
        line(),
        right.build(),
    ])
}
