use crate::buildable::Buildable;
use crate::document::Document;
use ruby_prism::ConstantTargetNode;

pub fn build_node(node: Option<&ConstantTargetNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
