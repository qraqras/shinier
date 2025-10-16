use crate::buildable::Buildable;
use crate::doc::{Doc, hardline, indent, none, sequence, text};
use ruby_prism::EnsureNode;

const ENSURE_KEYWORD: &str = "ensure";

pub fn build_node(node: Option<&EnsureNode>) -> Doc {
    match node {
        Some(node) => {
            let statements = node.statements();
            sequence(&[
                text(ENSURE_KEYWORD),
                indent(&[statements.build_with(Some(hardline()), None)]),
            ])
        }
        None => none(),
    }
}
