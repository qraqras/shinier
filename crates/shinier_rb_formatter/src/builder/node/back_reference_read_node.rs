use crate::doc::{Doc, text};
use crate::utility::constant_id_to_string;
use ruby_prism::BackReferenceReadNode;

pub fn build_node(node: Option<&BackReferenceReadNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    text(&constant_id_to_string(&name))
}
