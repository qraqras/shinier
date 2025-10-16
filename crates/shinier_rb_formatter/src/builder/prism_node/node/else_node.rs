use crate::buildable::Buildable;
use crate::doc::{Doc, hardline, indent, none, sequence, text};
use ruby_prism::ElseNode;

const ELSE_KEYWORD: &str = "else";

pub fn build_node(node: Option<&ElseNode>) -> Doc {
    match node {
        Some(node) => {
            let statements = node.statements();
            return sequence(&[
                text(ELSE_KEYWORD),
                indent(&[statements.build_with(Some(hardline()), None)]),
            ]);
        }
        None => return none(),
    }
}
