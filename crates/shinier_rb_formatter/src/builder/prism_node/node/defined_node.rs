use crate::buildable::Buildable;
use crate::doc::{Doc, group, text};
use crate::keyword::{DEFINED, PARENTHESES};
use ruby_prism::DefinedNode;

pub fn build_node(node: Option<&DefinedNode>) -> Doc {
    let node = node.unwrap();
    let value = node.value();
    group(&[
        text(DEFINED),
        text(PARENTHESES.0),
        value.build(),
        text(PARENTHESES.1),
    ])
}
