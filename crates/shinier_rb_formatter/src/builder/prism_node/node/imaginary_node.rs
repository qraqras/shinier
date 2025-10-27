use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::IMAGINARY;
use ruby_prism::ImaginaryNode;

pub fn build_node(node: Option<&ImaginaryNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let numeric = node.numeric();
    array(&[numeric.build(context), string(IMAGINARY)])
}
