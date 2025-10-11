use crate::doc::{Doc, group, indent, softline, text};
use crate::keyword::{BRACKETS, COMMA};
use crate::layout::separate_nodelist;
use ruby_prism::ArrayNode;

pub fn build_node(node: Option<&ArrayNode>) -> Doc {
    let node = node.unwrap();
    let elements = node.elements();
    group(&[
        text(BRACKETS.0),
        softline(),
        indent(&[group(&separate_nodelist(&elements, COMMA))]),
        softline(),
        text(BRACKETS.1),
    ])
}
