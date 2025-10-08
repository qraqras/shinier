use crate::builder::build;
use crate::doc::{Doc, sequence, text};
use ruby_prism::AssocSplatNode;

const SPLAT_PREFIX: &str = "**";

pub fn build_node(node: Option<&AssocSplatNode>) -> Doc {
    let node = node.unwrap();
    let value = node.value();
    match value {
        Some(value) => sequence(&[text(SPLAT_PREFIX), build(&value)]),
        None => text(SPLAT_PREFIX),
    }
}
