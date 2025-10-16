use crate::buildable::Buildable;
use crate::doc::Doc;
use ruby_prism::BlockLocalVariableNode;

pub fn build_node(node: Option<&BlockLocalVariableNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
