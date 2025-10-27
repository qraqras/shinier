use crate::BuildPrismNode;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::IMAGINARY;
use ruby_prism::Comments;
use ruby_prism::ImaginaryNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&ImaginaryNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let numeric = node.numeric();
    array(&[numeric.build(comments), string(IMAGINARY)])
}
