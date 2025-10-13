use crate::doc::{Doc, text_constant};
use ruby_prism::BackReferenceReadNode;

pub fn build_node(node: Option<&BackReferenceReadNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    text_constant(&name)
}
