use crate::Document;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_node_location;
use ruby_prism::SymbolNode;

pub fn build_symbol_node(node: &SymbolNode<'_>, context: &mut BuildContext) -> Document {
    build_node_location(&node.as_node(), context)
}
