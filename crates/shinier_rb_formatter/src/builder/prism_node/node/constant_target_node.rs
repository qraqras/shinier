use crate::buildable::Buildable;
use crate::doc::Doc;
use ruby_prism::ConstantTargetNode;

pub fn build_node(node: Option<&ConstantTargetNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
