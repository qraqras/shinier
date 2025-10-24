use crate::buildable::Buildable;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::NEXT;
use ruby_prism::NextNode;

pub fn build_node(node: Option<&NextNode>) -> Document {
    let node = node.unwrap();
    let arguments = node.arguments();
    group(array(&[string(NEXT), space(), arguments.build()]))
}
