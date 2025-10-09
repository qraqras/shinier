use crate::builder::node::parameters_node;
use crate::doc::{Doc, none};
use ruby_prism::BlockParametersNode;

pub fn build_node(node: Option<&BlockParametersNode>) -> Doc {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            parameters_node::build_node(parameters.as_ref())
        }
        None => none(),
    }
}
