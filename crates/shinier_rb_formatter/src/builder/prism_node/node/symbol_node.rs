use crate::doc::{Doc, text_from_u8};
use ruby_prism::SymbolNode;

pub fn build_node(node: Option<&SymbolNode>) -> Doc {
    let node = node.unwrap();
    text_from_u8(node.unescaped())
}
