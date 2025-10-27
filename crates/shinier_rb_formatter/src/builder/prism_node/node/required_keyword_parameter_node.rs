use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::{array, group, none, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::{ASTERISK, COLON};
use ruby_prism::RequiredKeywordParameterNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&RequiredKeywordParameterNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let is_repeated_parameter = node.is_repeated_parameter();
    let name = node.name();
    group(array(&[
        match is_repeated_parameter {
            true => string(ASTERISK),
            false => none(),
        },
        name.build(comments),
        string(COLON),
    ]))
}
