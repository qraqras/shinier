use crate::builder::build;
use crate::doc::{Doc, line, sequence, space, text, text_constant};
use ruby_prism::OptionalParameterNode;

const OPERATOR: &str = "=";

pub fn build_node(node: Option<&OptionalParameterNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    sequence(&[
        text_constant(&name),
        space(),
        text(OPERATOR),
        line(),
        build(&value),
    ])
}
