use crate::doc::{Doc, none, sequence, text, text_constant};
use ruby_prism::KeywordRestParameterNode;

const KEYWORD_REST_PARAMETER_PREFIX: &str = "**";

pub fn build_node(node: Option<&KeywordRestParameterNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    sequence(&[
        text(KEYWORD_REST_PARAMETER_PREFIX),
        match name {
            Some(name) => text_constant(&name),
            None => none(),
        },
    ])
}
