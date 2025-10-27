use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::WRITE_OPERATOR;
use ruby_prism::OptionalParameterNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&OptionalParameterNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    group(array(&[
        name.build(comments),
        space(),
        string(WRITE_OPERATOR),
        space(),
        value.build(comments),
    ]))
}
