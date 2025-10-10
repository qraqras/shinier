use crate::builder::node::parameters_node;
use crate::doc::{Doc, none, sequence, text};
use ruby_prism::BlockParametersNode;

const PARAMETERS_SEPARATOR: &str = "|";

pub fn build_node(node: Option<&BlockParametersNode>) -> Doc {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            sequence(&[
                text(PARAMETERS_SEPARATOR),
                parameters_node::build_node(parameters.as_ref()),
                text(PARAMETERS_SEPARATOR),
            ])
        }
        None => none(),
    }
}
