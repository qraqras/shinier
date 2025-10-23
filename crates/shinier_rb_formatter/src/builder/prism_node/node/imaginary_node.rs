use crate::buildable::Buildable;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::IMAGINARY;
use ruby_prism::ImaginaryNode;

pub fn build_node(node: Option<&ImaginaryNode>) -> Document {
    let node = node.unwrap();
    let numeric = node.numeric();
    array(&[numeric.build(), string(IMAGINARY)])
}
