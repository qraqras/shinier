use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::{END, WHILE};
use ruby_prism::WhileNode;

pub fn build_node(node: Option<&WhileNode>) -> Document {
    let node = node.unwrap();
    let is_begin_modifier = node.is_begin_modifier();
    let predicate = node.predicate();
    let statements = node.statements();
    match is_begin_modifier {
        true => group(array(&[
            statements.build(),
            space(),
            string(WHILE),
            space(),
            predicate.build(),
        ])),
        false => group(array(&[
            string(WHILE),
            space(),
            predicate.build(),
            indent(array(&[hardline(), statements.build()])),
            line(),
            string(END),
        ])),
    }
}
