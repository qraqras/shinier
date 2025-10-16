use crate::buildable::Buildable;
use crate::doc::{Doc, none, sequence, text};
use crate::keyword::PIPE;
use ruby_prism::BlockParametersNode;

pub fn build_node(node: Option<&BlockParametersNode>) -> Doc {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            sequence(&[text(PIPE), parameters.build(), text(PIPE)])
        }
        None => none(),
    }
}
