use crate::{BuildPrismNode, BuildPrismNodeList};
use crate::builder::builder::{array, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::SPLAT;
use ruby_prism::KeywordRestParameterNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&KeywordRestParameterNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    array(&[string(SPLAT), name.build(comments)])
}
