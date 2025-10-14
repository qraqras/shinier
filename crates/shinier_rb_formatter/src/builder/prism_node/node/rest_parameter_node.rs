use crate::builder::Buildable;
use crate::doc::{Doc, sequence, text};
use ruby_prism::RestParameterNode;

const REST_PARAMETER_PREFIX: &str = "*";

pub fn build_node(node: Option<&RestParameterNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    match name {
        Some(name) => sequence(&[text(REST_PARAMETER_PREFIX), name.build()]),
        None => text(REST_PARAMETER_PREFIX),
    }
}
