use crate::buildable::Buildable;
use crate::doc::Doc;
use ruby_prism::Node;

pub fn build_symbol_without_colon(symbol_node: &Node) -> Doc {
    match symbol_node.as_symbol_node() {
        Some(symbol_node) => symbol_node.unescaped().build(),
        None => panic!("Expected a SymbolNode"),
    }
}
