use crate::builder::Buildable;
use crate::doc::Doc;
use ruby_prism::ClassVariableReadNode;

pub fn build_node(node: Option<&ClassVariableReadNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
