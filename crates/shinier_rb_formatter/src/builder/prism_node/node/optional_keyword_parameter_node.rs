use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use ruby_prism::OptionalKeywordParameterNode;

const REPEATED_PARAMETER_PREFIX: &str = "*";
const REQUIRED_KEYWORD_PARAMETER_SUFFIX: &str = ":";

pub fn build_node(node: Option<&OptionalKeywordParameterNode>) -> Doc {
    let node = node.unwrap();
    let is_repeated_parameter = node.is_repeated_parameter();
    let name = node.name();
    let value = node.value();
    group(array(&[
        match is_repeated_parameter {
            true => string(REPEATED_PARAMETER_PREFIX),
            false => none(),
        },
        name.build(),
        string(REQUIRED_KEYWORD_PARAMETER_SUFFIX),
        line(),
        value.build(),
    ]))
}
