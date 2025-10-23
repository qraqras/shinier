use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use ruby_prism::RequiredParameterNode;

const REPEATED_PARAMETER_PREFIX: &str = "*";

pub fn build_node(node: Option<&RequiredParameterNode>) -> Document {
    match node {
        Some(node) => {
            let is_repeated_parameter = node.is_repeated_parameter();
            let name = node.name();
            match is_repeated_parameter {
                true => array(&[string(REPEATED_PARAMETER_PREFIX), name.build()]),
                false => name.build(),
            }
        }
        None => none(),
    }
}
