use crate::buildable::{Buildable, BuildableList};
use crate::doc::{Doc, group, indent, line, sequence, softline, text};
use crate::keyword::{BRACKETS, COMMA};
use ruby_prism::ArrayNode;

pub fn build_node(node: Option<&ArrayNode>) -> Doc {
    let node = node.unwrap();
    let elements = node.elements();
    let opening_loc = node.opening_loc();
    let closing_loc = node.closing_loc();
    let separator = sequence(&[text(COMMA), line()]);
    group(&[
        opening_loc.build_or(text(BRACKETS.0)),
        softline(),
        indent(&[elements.build(separator, group)]),
        softline(),
        closing_loc.build_or(text(BRACKETS.1)),
    ])
}
