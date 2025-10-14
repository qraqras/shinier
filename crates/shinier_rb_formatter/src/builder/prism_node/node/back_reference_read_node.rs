use crate::builder::Buildable;
use crate::doc::Doc;
use ruby_prism::BackReferenceReadNode;

pub fn build_node(node: Option<&BackReferenceReadNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
