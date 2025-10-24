use crate::buildable::Buildable;
use crate::builder::builder::{array, group, indent, space, string};
use crate::document::Document;
use crate::keyword::RETURN;
use ruby_prism::ReturnNode;

pub fn build_node(node: Option<&ReturnNode>) -> Document {
    let node = node.unwrap();
    let arguments = node.arguments();
    group(array(&[string(RETURN), space(), indent(arguments.build())]))
}
