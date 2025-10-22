use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::GlobalVariableReadNode;

pub fn build_node(node: Option<&GlobalVariableReadNode>) -> Document {
    let node = node.unwrap();
    node.name().build()
}
