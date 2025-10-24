use crate::builder::Buildable;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::SPLAT;
use ruby_prism::KeywordRestParameterNode;

pub fn build_node(node: Option<&KeywordRestParameterNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    array(&[string(SPLAT), name.build()])
}
