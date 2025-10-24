use crate::builder::Buildable;
use crate::builder::builder::{array, group, none, space, string};
use crate::document::Document;
use crate::keyword::{ASTERISK, COLON};
use ruby_prism::OptionalKeywordParameterNode;

pub fn build_node(node: Option<&OptionalKeywordParameterNode>) -> Document {
    let node = node.unwrap();
    let is_repeated_parameter = node.is_repeated_parameter();
    let name = node.name();
    let value = node.value();
    group(array(&[
        match is_repeated_parameter {
            true => string(ASTERISK),
            false => none(),
        },
        name.build(),
        string(COLON),
        space(),
        value.build(),
    ]))
}
