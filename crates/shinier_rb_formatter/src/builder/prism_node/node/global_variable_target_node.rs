use crate::Buildable;
use crate::document::Document;
use ruby_prism::GlobalVariableTargetNode;

pub fn build_node(node: Option<&GlobalVariableTargetNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
