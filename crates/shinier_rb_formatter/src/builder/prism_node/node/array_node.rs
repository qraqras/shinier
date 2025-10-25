use crate::buildable::BuildableList;
use crate::builder::builder::{array, group, indent, line, softline, string};
use crate::document::Document;
use crate::keyword::{BRACKETS, COMMA};
use ruby_prism::ArrayNode;

pub fn build_node(node: Option<&ArrayNode>) -> Document {
    let node = node.unwrap();
    let elements = node.elements();
    let separator = array(&[string(COMMA), line()]);
    group(array(&[
        string(BRACKETS.0),
        indent(array(&[softline(), elements.build(separator)])),
        softline(),
        string(BRACKETS.1),
    ]))
}
