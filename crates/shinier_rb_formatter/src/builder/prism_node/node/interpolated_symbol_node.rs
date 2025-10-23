use crate::buildable::BuildableList;
use crate::builder::builder::{array, group, none, string};
use crate::document::Document;
use crate::keyword::{COLON, DOUBLE_QUOTE};
use ruby_prism::InterpolatedSymbolNode;

pub fn build_node(node: Option<&InterpolatedSymbolNode>) -> Document {
    let node = node.unwrap();
    let parts = node.parts();
    group(parts.build_with(
        none(),
        Some(array(&[string(COLON), string(DOUBLE_QUOTE)])),
        Some(string(DOUBLE_QUOTE)),
    ))
}
