use crate::builder::Buildable;
use crate::doc::{Doc, sequence, text};
use crate::keyword::COLON;
use ruby_prism::SymbolNode;

pub fn build_node(node: Option<&SymbolNode>) -> Doc {
    let node = node.unwrap();
    let unescaped = node.unescaped();
    sequence(&[text(COLON), unescaped.build()])
}
