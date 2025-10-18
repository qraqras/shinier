use crate::buildable::Buildable;
use crate::doc::{Doc, group, space, text};
use crate::keyword::{DOUBLE_DOT, TRIPLE_DOT};
use ruby_prism::FlipFlopNode;

pub fn build_node(node: Option<&FlipFlopNode>) -> Doc {
    let node = node.unwrap();
    let is_exclude_end = node.is_exclude_end();
    let left = node.left();
    let right = node.right();
    group(&[
        left.build(),
        space(),
        match is_exclude_end {
            true => text(TRIPLE_DOT),
            false => text(DOUBLE_DOT),
        },
        space(),
        right.build(),
    ])
}
