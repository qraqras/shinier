use crate::builder::Buildable;
use crate::doc::{Doc, fill, line, space, text};
use ruby_prism::OptionalParameterNode;

const OPERATOR: &str = "=";

pub fn build_node(node: Option<&OptionalParameterNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    let value = node.value();
    fill(&[name.build(), space(), text(OPERATOR), line(), value.build()])
}
