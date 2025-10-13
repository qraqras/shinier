use crate::doc::{Doc, sequence, text, text_constant};
use ruby_prism::RestParameterNode;

const REST_PARAMETER_PREFIX: &str = "*";

pub fn build_node(node: Option<&RestParameterNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    match name {
        Some(name) => sequence(&[text(REST_PARAMETER_PREFIX), text_constant(&name)]),
        None => text(REST_PARAMETER_PREFIX),
    }
}
