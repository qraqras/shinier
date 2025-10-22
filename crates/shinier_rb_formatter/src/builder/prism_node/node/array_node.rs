use crate::buildable::{Buildable, BuildableList};
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::{BRACKETS, COMMA};
use ruby_prism::ArrayNode;

pub fn build_node(node: Option<&ArrayNode>) -> Doc {
    let node = node.unwrap();
    let elements = node.elements();
    let opening_loc = node.opening_loc();
    let closing_loc = node.closing_loc();
    let separator = array(&[string(COMMA), line()]);
    group(array(&[
        opening_loc.build_or(string(BRACKETS.0)),
        softline(),
        indent(array(&[elements.build(separator, array)])),
        softline(),
        closing_loc.build_or(string(BRACKETS.1)),
    ]))
}
