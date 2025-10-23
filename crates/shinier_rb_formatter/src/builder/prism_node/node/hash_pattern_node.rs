use crate::buildable::{Buildable, BuildableList};
use crate::builder::builder::{array, group, indent, line, softline, string};
use crate::document::Document;
use crate::helper::separate_docs::separate_docs;
use crate::keyword::{BRACES, BRACKETS, COMMA};
use ruby_prism::*;

pub fn build_node(node: Option<&HashPatternNode>) -> Document {
    let node = node.unwrap();
    let constant = node.constant();
    let elements = node.elements();
    let rest = node.rest();

    let separator = array(&[string(COMMA), line()]);

    match constant {
        Some(constant) => group(array(&[
            constant.build(),
            string(BRACKETS.0),
            indent(array(&[
                softline(),
                array(&separate_docs(
                    &[elements.build(separator.clone()), rest.build()],
                    separator.clone(),
                )),
            ])),
            softline(),
            string(BRACKETS.1),
        ])),
        None => group(array(&[
            string(BRACES.0),
            indent(array(&[
                line(),
                array(&separate_docs(
                    &[elements.build(separator.clone()), rest.build()],
                    separator.clone(),
                )),
            ])),
            line(),
            string(BRACES.1),
        ])),
    }
}
