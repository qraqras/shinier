use crate::builder::build;
use crate::doc::{Doc, group, text};
use crate::prism_utility::constant_id_to_string;
use ruby_prism::CallTargetNode;

pub fn build_node(node: Option<&CallTargetNode>) -> Doc {
    let node = node.unwrap();
    let receiver = node.receiver();
    let name = constant_id_to_string(&node.name());

    let mut vec = Vec::new();
    vec.push(build(&receiver));
    if node.is_safe_navigation() {
        vec.push(text("&."));
    } else {
        vec.push(text("."));
    }
    vec.push(text(name));
    group(&vec)
}
