use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::BackReferenceReadNode;

pub fn build_node(node: Option<&BackReferenceReadNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build()
}
