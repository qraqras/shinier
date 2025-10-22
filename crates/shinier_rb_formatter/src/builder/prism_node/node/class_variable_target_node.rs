use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::ClassVariableTargetNode;

pub fn build_node(node: Option<&ClassVariableTargetNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
