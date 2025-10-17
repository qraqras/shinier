use crate::buildable::Buildable;
use crate::doc::{Doc, sequence, text};
use crate::keyword::{BRACES, HASH};
use ruby_prism::EmbeddedStatementsNode;

pub fn build_node(node: Option<&EmbeddedStatementsNode>) -> Doc {
    let node = node.unwrap();
    let statements = node.statements();
    statements.build_with(
        Some(sequence(&[text(HASH), text(BRACES.0)])),
        Some(text(BRACES.1)),
    )
}
