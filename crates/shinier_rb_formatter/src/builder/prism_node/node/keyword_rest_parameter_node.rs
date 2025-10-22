use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use ruby_prism::KeywordRestParameterNode;

const KEYWORD_REST_PARAMETER_PREFIX: &str = "**";

pub fn build_node(node: Option<&KeywordRestParameterNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    array(&[
        string(KEYWORD_REST_PARAMETER_PREFIX),
        match name {
            Some(name) => name.build(),
            None => none(),
        },
    ])
}
