use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{END, WHILE};
use ruby_prism::WhileNode;

pub fn build_node(node: Option<&WhileNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let is_begin_modifier = node.is_begin_modifier();
    let predicate = node.predicate();
    let statements = node.statements();
    match is_begin_modifier {
        true => group(array(&[
            statements.build(context),
            space(),
            string(WHILE),
            space(),
            predicate.build(context),
        ])),
        false => group(array(&[
            string(WHILE),
            space(),
            predicate.build(context),
            indent(array(&[hardline(), statements.build(context)])),
            line(),
            string(END),
        ])),
    }
}
