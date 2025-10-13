use crate::builder::prism_node::node::parameters_node;
use crate::doc::{Doc, none, sequence, text};
use crate::keyword::PIPE;
use ruby_prism::BlockParametersNode;

pub fn build_node(node: Option<&BlockParametersNode>) -> Doc {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            sequence(&[
                text(PIPE),
                parameters_node::build_node(parameters.as_ref()),
                text(PIPE),
            ])
        }
        None => none(),
    }
}
