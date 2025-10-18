use crate::buildable::Buildable;
use crate::doc::{Doc, group, hardline, indent, text};
use crate::keyword::{BEGIN, END};
use ruby_prism::BeginNode;

pub fn build_node(node: Option<&BeginNode>) -> Doc {
    let node = node.unwrap();
    let statements = node.statements();
    let rescue_clause = node.rescue_clause();
    let else_clause = node.else_clause();
    let ensure_clause = node.ensure_clause();
    group(&[
        text(BEGIN),
        indent(&[statements.build_with(Some(hardline()), None)]),
        rescue_clause.build_with(Some(hardline()), None),
        else_clause.build_with(Some(hardline()), None),
        ensure_clause.build_with(Some(hardline()), None),
        hardline(),
        text(END),
    ])
}
