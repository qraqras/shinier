use crate::builder::Buildable;
use crate::doc::Doc;
use ruby_prism::SymbolNode;

pub fn build_node(node: Option<&SymbolNode>) -> Doc {
    let node = node.unwrap();
    let unescaped = node.unescaped();
    unescaped.build()
}
