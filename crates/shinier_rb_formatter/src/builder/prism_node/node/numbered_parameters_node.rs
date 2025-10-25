use crate::buildable::Buildable;
use crate::document::Document;
use ruby_prism::NumberedParametersNode;

pub fn build_node(node: Option<&NumberedParametersNode>) -> Document {
    let node = node.unwrap();
    let maximum = node.maximum();
    maximum.build()
}
