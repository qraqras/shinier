use crate::builder::pattern::receiver_pattern::build_receiver_pattern;
use crate::doc::{Doc, sequence, text_constant};
use ruby_prism::CallTargetNode;

pub fn build_node(node: Option<&CallTargetNode>) -> Doc {
    let node = node.unwrap();
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let name = node.name();
    sequence(&[
        build_receiver_pattern(Some(&receiver), is_safe_navigation),
        text_constant(&name),
    ])
}
