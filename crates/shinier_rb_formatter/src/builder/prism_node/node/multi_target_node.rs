use crate::buildable::{Buildable, BuildableList};
use crate::builder::builder::{array, group, indent, line, softline, string};
use crate::builder::helper::separate_docs::separate_docs;
use crate::document::Document;
use crate::keyword::{COMMA, PARENTHESES};
use ruby_prism::MultiTargetNode;

pub fn build_node(node: Option<&MultiTargetNode>) -> Document {
    let node = node.unwrap();
    let lefts = node.lefts();
    let rest = node.rest();
    let rights = node.rights();

    let separator = array(&[string(COMMA), line()]);
    group(array(&[
        string(PARENTHESES.0),
        indent(array(&[
            softline(),
            group(array(&separate_docs(
                &[
                    lefts.build(separator.clone()),
                    rest.build(),
                    rights.build(separator.clone()),
                ],
                separator.clone(),
            ))),
        ])),
        softline(),
        string(PARENTHESES.1),
    ]))
}
