use crate::builder::build;
use crate::doc::{Doc, fill, line, none, text, text_constant};
use ruby_prism::OptionalKeywordParameterNode;

const REPEATED_PARAMETER_PREFIX: &str = "*";
const REQUIRED_KEYWORD_PARAMETER_SUFFIX: &str = ":";

pub fn build_node(node: Option<&OptionalKeywordParameterNode>) -> Doc {
    let node = node.unwrap();
    let is_repeated_parameter = node.is_repeated_parameter();
    let name = node.name();
    let value = node.value();
    fill(&[
        match is_repeated_parameter {
            true => text(REPEATED_PARAMETER_PREFIX),
            false => none(),
        },
        text_constant(&name),
        text(REQUIRED_KEYWORD_PARAMETER_SUFFIX),
        line(),
        build(&value),
    ])
}
