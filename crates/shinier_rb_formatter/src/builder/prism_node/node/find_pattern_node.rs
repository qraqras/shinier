use crate::buildable::{Buildable, BuildableList};
use crate::builder::builder::*;
use crate::document::*;
use crate::helper::separate_docs::separate_docs;
use crate::keyword::{BRACKETS, COMMA, PARENTHESES};
use ruby_prism::FindPatternNode;

pub fn build_node(node: Option<&FindPatternNode>) -> Doc {
    let node = node.unwrap();
    let constant = node.constant();
    let left = node.left();
    let requireds = node.requireds();
    let right = node.right();

    let separator = array(&[string(COMMA), line()]);
    let elements = separate_docs(
        &[
            left.as_node().build(),
            requireds.build(separator.clone(), array),
            right.build(),
        ],
        separator.clone(),
    );
    match constant {
        Some(constant) => group(array(&[
            constant.build(),
            string(PARENTHESES.0),
            indent(array(&[group(array(&elements))])),
            string(PARENTHESES.1),
        ])),
        None => group(array(&[
            string(BRACKETS.0),
            indent(array(&[group(array(&elements))])),
            string(BRACKETS.1),
        ])),
    }
}
