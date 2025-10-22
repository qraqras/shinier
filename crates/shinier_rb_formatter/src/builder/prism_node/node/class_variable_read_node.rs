use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::ClassVariableReadNode;

pub fn build_node(node: Option<&ClassVariableReadNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
