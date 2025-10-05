use crate::builder::build;
use crate::doc::{Doc, group, text};
use crate::utility::constant_id_to_string;
use ruby_prism::CallAndWriteNode;

const OPERATOR: &str = "&&=";

pub fn build_node(node: &CallAndWriteNode) -> Doc {
    let receiver = node.receiver();
    let read_name = constant_id_to_string(&node.read_name());
    let value = &node.value();

    let mut vec = Vec::new();
    if let Some(receiver) = &receiver {
        vec.push(build(receiver));
        if node.is_safe_navigation() {
            vec.push(text("&."));
        } else {
            vec.push(text("."));
        };
    };
    vec.push(text(read_name));
    vec.push(text(" "));
    vec.push(text(OPERATOR));
    vec.push(text(" "));
    vec.push(build(&value));
    group(vec)
}
