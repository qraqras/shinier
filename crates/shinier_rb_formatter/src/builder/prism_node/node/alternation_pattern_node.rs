use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::ALTERNATION;
use ruby_prism::AlternationPatternNode;

pub fn build_node(node: Option<&AlternationPatternNode>) -> Doc {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    array(&[
        left.build(),
        space(),
        string(ALTERNATION),
        line(),
        right.build(),
    ])
}
