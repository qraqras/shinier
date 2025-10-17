use crate::buildable::Buildable;
use crate::doc::{Doc, line, sequence, text};
use crate::keyword::{DEFINED, PARENTHESES};
use ruby_prism::DefinedNode;

pub fn build_node(node: Option<&DefinedNode>) -> Doc {
    let node = node.unwrap();
    let value = node.value();
    sequence(&[
        text(DEFINED),
        text(PARENTHESES.0),
        value.build(),
        text(PARENTHESES.1),
    ])
}
