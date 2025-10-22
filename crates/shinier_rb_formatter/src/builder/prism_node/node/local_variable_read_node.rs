use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::LocalVariableReadNode;

pub fn build_node(node: Option<&LocalVariableReadNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
