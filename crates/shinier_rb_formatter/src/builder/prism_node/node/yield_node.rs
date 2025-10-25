use crate::buildable::Buildable;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::YIELD;
use ruby_prism::YieldNode;

pub fn build_node(node: Option<&YieldNode>) -> Document {
    let node = node.unwrap();
    group(array(&[string(YIELD), space(), node.arguments().build()]))
}
