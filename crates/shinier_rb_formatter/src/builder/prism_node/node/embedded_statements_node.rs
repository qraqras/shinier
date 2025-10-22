use crate::buildable::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::{BRACES, HASH};
use ruby_prism::EmbeddedStatementsNode;

pub fn build_node(node: Option<&EmbeddedStatementsNode>) -> Doc {
    let node = node.unwrap();
    let statements = node.statements();
    statements.build_with(
        Some(array(&[string(HASH), string(BRACES.0)])),
        Some(string(BRACES.1)),
    )
}
