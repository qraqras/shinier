use crate::builder::build;
use crate::doc::{Doc, group, space, text};
use crate::prism_utility::constant_id_to_string;
use ruby_prism::CallOrWriteNode;

const OPERATOR: &str = "||=";

pub fn build_node(node: Option<&CallOrWriteNode>) -> Doc {
    let node = node.unwrap();
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
    vec.push(space());
    vec.push(text(OPERATOR));
    vec.push(space());
    vec.push(build(&value));
    group(&vec)
}
