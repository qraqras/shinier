use crate::buildable::Buildable;
use crate::doc::{Doc, text};
use crate::keyword::HASH;
use ruby_prism::EmbeddedVariableNode;

pub fn build_node(node: Option<&EmbeddedVariableNode>) -> Doc {
    let node = node.unwrap();
    let variable = node.variable();
    variable.build_with(Some(text(HASH)), None)
}
