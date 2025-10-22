use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use ruby_prism::OptionalParameterNode;

const OPERATOR: &str = "=";

pub fn build_node(node: Option<&OptionalParameterNode>) -> Document {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    group(array(&[
        name.build(),
        space(),
        string(OPERATOR),
        line(),
        value.build(),
    ]))
}
