use crate::buildable::BuildableList;
use crate::doc::{Doc, none, sequence, text};
use crate::keyword::DOUBLE_QUOTE;
use ruby_prism::InterpolatedStringNode;

pub fn build_node(node: Option<&InterpolatedStringNode>) -> Doc {
    let node = node.unwrap();
    let parts = node.parts();
    parts.build_with(
        none(),
        sequence,
        Some(text(DOUBLE_QUOTE)),
        Some(text(DOUBLE_QUOTE)),
    )
}
