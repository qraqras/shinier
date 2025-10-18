use crate::buildable::Buildable;
use crate::doc::{Doc, group, hardline, indent, none, text};
use crate::keyword::ELSE;
use ruby_prism::ElseNode;

pub fn build_node(node: Option<&ElseNode>) -> Doc {
    match node {
        Some(node) => {
            let statements = node.statements();
            group(&[
                text(ELSE),
                indent(&[statements.build_with(Some(hardline()), None)]),
            ])
        }
        None => none(),
    }
}
