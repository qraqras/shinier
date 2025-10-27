use crate::builder::builder::{array, group, none, space, string};
use crate::document::Document;
use crate::keyword::{ASTERISK, COLON};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::OptionalKeywordParameterNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&OptionalKeywordParameterNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let is_repeated_parameter = node.is_repeated_parameter();
    let name = node.name();
    let value = node.value();
    group(array(&[
        match is_repeated_parameter {
            true => string(ASTERISK),
            false => none(),
        },
        name.build(comments),
        string(COLON),
        space(),
        value.build(comments),
    ]))
}
