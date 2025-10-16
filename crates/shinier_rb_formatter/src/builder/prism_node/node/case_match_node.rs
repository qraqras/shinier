use crate::buildable::{Buildable, BuildableList};
use crate::doc::*;
use crate::keyword::{CASE, COMMA, END, SEMI_COLON};
use ruby_prism::CaseMatchNode;

pub fn build_node(node: Option<&CaseMatchNode>) -> Doc {
    let node = node.unwrap();
    let predicate = node.predicate();
    let conditions = node.conditions();
    let else_clause = node.else_clause();
    sequence(&[
        text(CASE),
        space(),
        predicate.build(),
        conditions.build_with(hardline(), sequence, Some(hardline()), None),
        else_clause.build_with(Some(hardline()), None),
        hardline(),
        text(END),
    ])
}
