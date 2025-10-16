use crate::BuildableList;
use crate::doc::{Doc, group, indent, line, sequence, softline, text};
use crate::keyword::{BRACKETS, COMMA};
use ruby_prism::ArrayNode;

pub fn build_node(node: Option<&ArrayNode>) -> Doc {
    let node = node.unwrap();
    let elements = node.elements();
    let separator = sequence(&[text(COMMA), line()]);
    group(&[
        text(BRACKETS.0),
        softline(),
        indent(&[elements.build(separator, group)]),
        softline(),
        text(BRACKETS.1),
    ])
}
