use crate::builder::BuildableList;
use crate::builder::builder::{array, group, indent, line, string};
use crate::document::Document;
use crate::keyword::{BRACES, COMMA};
use ruby_prism::HashNode;

pub fn build_node(node: Option<&HashNode>) -> Document {
    let node = node.unwrap();
    let elements = node.elements();
    group(array(&[
        string(BRACES.0),
        indent(array(&[
            line(),
            elements.build(array(&[string(COMMA), line()])),
        ])),
        line(),
        string(BRACES.1),
    ]))
}
