use crate::builder::node::{else_node, ensure_node, rescue_node, statements_node};
use crate::builder::{build, build_optional};
use crate::doc::*;
use ruby_prism::*;

const BEGIN_KEYWORD: &str = "begin";
const END_KEYWORD: &str = "end";

pub fn build_node(node: Option<&BeginNode>) -> Doc {
    let node = node.unwrap();
    let statements = node.statements();
    let rescue_clause = node.rescue_clause();
    let else_clause = node.else_clause();
    let ensure_clause = node.ensure_clause();

    sequence(&[
        text(BEGIN_KEYWORD),
        hardline(),
        indent(&[statements_node::build_node(statements.as_ref())]),
        rescue_node::build_node(rescue_clause.as_ref()),
        else_node::build_node(else_clause.as_ref()),
        ensure_node::build_node(ensure_clause.as_ref()),
        text(END_KEYWORD),
        hardline(),
    ])
}
