use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use ruby_prism::OptionalParameterNode;

const OPERATOR: &str = "=";

pub fn build_node(node: Option<&OptionalParameterNode>) -> Doc {
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
