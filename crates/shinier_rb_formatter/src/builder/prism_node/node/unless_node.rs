use crate::buildable::Buildable;
use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{END, UNLESS};
use ruby_prism::UnlessNode;

pub fn build_node(node: Option<&UnlessNode>) -> Document {
    let node = node.unwrap();
    let predicate = node.predicate();
    let statements = node.statements();
    let else_clause = node.else_clause();
    group(array(&[
        string(UNLESS),
        space(),
        predicate.build(),
        indent(statements.build_with(Some(hardline()), None)),
        else_clause.build_with(Some(hardline()), None),
        line(),
        string(END),
    ]))
}
