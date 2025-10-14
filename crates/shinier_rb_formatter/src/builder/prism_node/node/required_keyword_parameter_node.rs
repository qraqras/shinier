use crate::builder::Buildable;
use crate::doc::{Doc, none, sequence, text};
use ruby_prism::RequiredKeywordParameterNode;

const REPEATED_PARAMETER_PREFIX: &str = "*";
const REQUIRED_KEYWORD_PARAMETER_SUFFIX: &str = ":";

pub fn build_node(node: Option<&RequiredKeywordParameterNode>) -> Doc {
    let node = node.unwrap();
    let is_repeated_parameter = node.is_repeated_parameter();
    let name = node.name();
    sequence(&[
        match is_repeated_parameter {
            true => text(REPEATED_PARAMETER_PREFIX),
            false => none(),
        },
        name.build(),
        text(REQUIRED_KEYWORD_PARAMETER_SUFFIX),
    ])
}
