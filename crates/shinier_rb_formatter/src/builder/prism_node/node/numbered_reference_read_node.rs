use crate::doc::{Doc, sequence, text};
use ruby_prism::NumberedReferenceReadNode;

const REFERENCE_PREFIX: &str = "$";

pub fn build_node(node: Option<&NumberedReferenceReadNode>) -> Doc {
    let node = node.unwrap();
    let number = node.number();
    sequence(&[text(REFERENCE_PREFIX), text(number.to_string())])
}
