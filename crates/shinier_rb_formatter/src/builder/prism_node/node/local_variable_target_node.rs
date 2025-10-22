use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::LocalVariableTargetNode;

pub fn build_node(node: Option<&LocalVariableTargetNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
