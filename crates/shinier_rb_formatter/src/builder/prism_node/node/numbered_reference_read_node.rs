use crate::BuildPrismNode;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::DOLLAR;
use ruby_prism::Comments;
use ruby_prism::NumberedReferenceReadNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&NumberedReferenceReadNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let number = node.number();
    array(&[string(DOLLAR), string(number.to_string())])
}
