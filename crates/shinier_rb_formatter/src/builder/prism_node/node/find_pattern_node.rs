use crate::buildable::{Buildable, BuildableList};
use crate::doc::{Doc, group, indent, line, sequence, text};
use crate::helper::separate_docs::separate_docs;
use crate::keyword::{BRACKETS, COMMA, PARENTHESES};
use ruby_prism::FindPatternNode;

pub fn build_node(node: Option<&FindPatternNode>) -> Doc {
    let node = node.unwrap();
    let constant = node.constant();
    let left = node.left();
    let requireds = node.requireds();
    let right = node.right();

    let separator = sequence(&[text(COMMA), line()]);
    let elements = separate_docs(
        &[
            left.as_node().build(),
            requireds.build(separator.clone(), sequence),
            right.build(),
        ],
        separator.clone(),
    );
    match constant {
        Some(constant) => group(&[
            constant.build(),
            text(PARENTHESES.0),
            indent(&[group(&elements)]),
            text(PARENTHESES.1),
        ]),
        None => group(&[
            text(BRACKETS.0),
            indent(&[group(&elements)]),
            text(BRACKETS.1),
        ]),
    }
}
