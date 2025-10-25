use crate::buildable::Buildable;
use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{END, UNTIL};
use ruby_prism::UntilNode;

pub fn build_node(node: Option<&UntilNode>) -> Document {
    let node = node.unwrap();
    let is_begin_modifier = node.is_begin_modifier();
    let predicate = node.predicate();
    let statements = node.statements();
    match is_begin_modifier {
        true => group(array(&[
            statements.build(),
            space(),
            string(UNTIL),
            space(),
            predicate.build(),
        ])),
        false => group(array(&[
            string(UNTIL),
            space(),
            predicate.build(),
            indent(array(&[hardline(), statements.build()])),
            line(),
            string(END),
        ])),
    }
}
