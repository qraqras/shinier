use crate::builder::builder::*;
use crate::document::*;
use ruby_prism::NumberedReferenceReadNode;

const REFERENCE_PREFIX: &str = "$";

pub fn build_node(node: Option<&NumberedReferenceReadNode>) -> Doc {
    let node = node.unwrap();
    let number = node.number();
    array(&[string(REFERENCE_PREFIX), string(number.to_string())])
}
