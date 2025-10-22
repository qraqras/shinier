use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use ruby_prism::RestParameterNode;

const REST_PARAMETER_PREFIX: &str = "*";

pub fn build_node(node: Option<&RestParameterNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    match name {
        Some(name) => array(&[string(REST_PARAMETER_PREFIX), name.build()]),
        None => string(REST_PARAMETER_PREFIX),
    }
}
