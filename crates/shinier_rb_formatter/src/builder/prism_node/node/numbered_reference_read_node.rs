use crate::BuildContext;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::DOLLAR;
use ruby_prism::NumberedReferenceReadNode;

pub fn build_node(
    node: Option<&NumberedReferenceReadNode>,
    _context: &mut BuildContext,
) -> Document {
    let node = node.unwrap();
    let number = node.number();
    array(&[string(DOLLAR), string(number.to_string())])
}
