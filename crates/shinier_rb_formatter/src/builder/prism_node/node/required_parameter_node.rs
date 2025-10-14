use crate::builder::Buildable;
use crate::doc::{Doc, sequence, text};
use ruby_prism::RequiredParameterNode;

const REPEATED_PARAMETER_PREFIX: &str = "*";

pub fn build_node(node: Option<&RequiredParameterNode>) -> Doc {
    let node = node.unwrap();
    let is_repeated_parameter = node.is_repeated_parameter();
    let name = node.name();
    match is_repeated_parameter {
        true => sequence(&[text(REPEATED_PARAMETER_PREFIX), name.build()]),
        false => name.build(),
    }
}
