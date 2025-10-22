use crate::builder::Buildable;
use crate::builder::builder::{array, line, space, string};
use crate::document::Document;
use crate::keyword::ALTERNATION;
use ruby_prism::AlternationPatternNode;

pub fn build_node(node: Option<&AlternationPatternNode>) -> Document {
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
