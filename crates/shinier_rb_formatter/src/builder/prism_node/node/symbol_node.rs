use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::COLON;
use ruby_prism::SymbolNode;

pub fn build_node(node: Option<&SymbolNode>) -> Doc {
    let node = node.unwrap();
    let unescaped = node.unescaped();
    array(&[string(COLON), unescaped.build()])
}
