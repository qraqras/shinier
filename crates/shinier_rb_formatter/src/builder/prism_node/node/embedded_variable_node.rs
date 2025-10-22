use crate::buildable::Buildable;
use crate::document::*;
use crate::builder::builder::*;
use crate::keyword::HASH;
use ruby_prism::EmbeddedVariableNode;

pub fn build_node(node: Option<&EmbeddedVariableNode>) -> Doc {
    let node = node.unwrap();
    let variable = node.variable();
    variable.build_with(Some(string(HASH)), None)
}
