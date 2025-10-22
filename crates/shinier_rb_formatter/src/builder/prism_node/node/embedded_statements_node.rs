use crate::buildable::Buildable;
use crate::builder::builder::{array, group, string};
use crate::document::Document;
use crate::keyword::{BRACES, HASH};
use ruby_prism::EmbeddedStatementsNode;

pub fn build_node(node: Option<&EmbeddedStatementsNode>) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    group(statements.build_with(
        Some(array(&[string(HASH), string(BRACES.0)])),
        Some(string(BRACES.1)),
    ))
}
