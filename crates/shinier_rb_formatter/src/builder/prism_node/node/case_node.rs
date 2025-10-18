use crate::buildable::{Buildable, BuildableList};
use crate::doc::{Doc, group, hardline, space, text};
use crate::keyword::{CASE, END};
use ruby_prism::*;

pub fn build_node(node: Option<&CaseNode>) -> Doc {
    let node = node.unwrap();
    let predicate = node.predicate();
    let conditions = node.conditions();
    let else_clause = node.else_clause();
    group(&[
        text(CASE),
        predicate.build_with(Some(space()), None),
        conditions.build_with(hardline(), group, Some(hardline()), None),
        else_clause.build_with(Some(hardline()), None),
        hardline(),
        text(END),
    ])
}
