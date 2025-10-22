use crate::buildable::{Buildable, BuildableList};
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::{CASE, END};
use ruby_prism::CaseMatchNode;

pub fn build_node(node: Option<&CaseMatchNode>) -> Doc {
    let node = node.unwrap();
    let predicate = node.predicate();
    let conditions = node.conditions();
    let else_clause = node.else_clause();
    group(array(&[
        string(CASE),
        predicate.build_with(Some(space()), None),
        conditions.build_with(hardline(), array, Some(hardline()), None),
        else_clause.build_with(Some(hardline()), None),
        hardline(),
        string(END),
    ]))
}
