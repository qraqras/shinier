use crate::buildable::Buildable;
use crate::doc::{Doc, group, hardline, indent, none, text};
use crate::keyword::ENSURE;
use ruby_prism::EnsureNode;

pub fn build_node(node: Option<&EnsureNode>) -> Doc {
    match node {
        Some(node) => {
            let statements = node.statements();
            group(&[
                text(ENSURE),
                indent(&[statements.build_with(Some(hardline()), None)]),
            ])
        }
        None => none(),
    }
}
