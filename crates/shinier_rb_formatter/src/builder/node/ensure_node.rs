use crate::builder::node::statements_node;
use crate::doc::{Doc, hardline, indent, none, sequence, text};
use ruby_prism::EnsureNode;

const ENSURE_KEYWORD: &str = "ensure";

pub fn build_node(node: Option<&EnsureNode>) -> Doc {
    match node {
        Some(node) => {
            let statements = node.statements();
            sequence(&[
                text(ENSURE_KEYWORD),
                indent(&[
                    hardline(),
                    statements_node::build_node(statements.as_ref()),
                    hardline(),
                ]),
            ])
        }
        None => none(),
    }
}
