use crate::buildable::BuildableList;
use crate::builder::builder::{group, none, string};
use crate::document::Document;
use crate::keyword::BACK_QUOTE;
use ruby_prism::InterpolatedXStringNode;

pub fn build_node(node: Option<&InterpolatedXStringNode>) -> Document {
    let node = node.unwrap();
    let parts = node.parts();
    group(parts.build_with(none(), Some(string(BACK_QUOTE)), Some(string(BACK_QUOTE))))
}
