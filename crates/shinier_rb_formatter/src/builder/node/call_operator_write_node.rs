use crate::builder::build;
use crate::doc::{Doc, group, text};
use crate::utility::constant_id_to_string;
use ruby_prism::CallOperatorWriteNode;

pub fn build_node(node: &CallOperatorWriteNode) -> Doc {
    let receiver = node.receiver();
    let read_name = constant_id_to_string(&node.read_name());
    let binary_operator = constant_id_to_string(&node.binary_operator());
    let value = &node.value();

    let mut vec = Vec::new();

    if let Some(receiver) = &receiver {
        vec.push(build(receiver));
        if node.is_safe_navigation() {
            vec.push(text("&."));
        } else {
            vec.push(text("."));
        }
    }
    vec.push(text(&read_name));
    vec.push(text(" "));
    vec.push(text(&binary_operator));
    vec.push(text("="));
    vec.push(text(" "));
    vec.push(build(&value));
    group(&vec)
}
