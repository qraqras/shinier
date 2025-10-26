use crate::builder::Buildable;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::ALTERNATION;
use ruby_prism::AlternationPatternNode;

pub fn build_node(node: Option<&AlternationPatternNode>) -> Document {
    let node = node.unwrap();
    let left = node.left();
    let right = node.right();
    group(array(&[
        left.build(),
        space(),
        string(ALTERNATION),
        space(),
        right.build(),
    ]))
}
