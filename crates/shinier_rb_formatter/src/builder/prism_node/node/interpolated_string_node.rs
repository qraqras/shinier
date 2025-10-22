use crate::buildable::BuildableList;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::DOUBLE_QUOTE;
use ruby_prism::InterpolatedStringNode;

pub fn build_node(node: Option<&InterpolatedStringNode>) -> Doc {
    let node = node.unwrap();
    let parts = node.parts();
    parts.build_with(
        none(),
        array,
        Some(string(DOUBLE_QUOTE)),
        Some(string(DOUBLE_QUOTE)),
    )
}
