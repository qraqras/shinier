use crate::doc::{Doc, none, sequence, text, text_constant};
use ruby_prism::BlockParameterNode;

const BLOCK_PARAMETER_PREFIX: &str = "&";

pub fn build_node(node: Option<&BlockParameterNode>) -> Doc {
    match node {
        Some(node) => {
            let name = node.name();
            match name {
                Some(name) => sequence(&[text(BLOCK_PARAMETER_PREFIX), text_constant(&name)]),
                None => none(),
            }
        }
        None => none(),
    }
}
