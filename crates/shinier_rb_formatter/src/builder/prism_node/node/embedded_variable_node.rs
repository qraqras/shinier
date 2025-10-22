use crate::buildable::Buildable;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::HASH;
use ruby_prism::EmbeddedVariableNode;

pub fn build_node(node: Option<&EmbeddedVariableNode>) -> Document {
    let node = node.unwrap();
    let variable = node.variable();
    variable.build_with(Some(string(HASH)), None)
}
