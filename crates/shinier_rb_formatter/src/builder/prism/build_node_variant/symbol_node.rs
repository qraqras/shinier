use crate::Document;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_node_location;
use crate::builder::prism::layout_node_variant::symbol_node::LayoutParamSymbolNode;
use crate::builder::prism::layout_node_variant::symbol_node::layout_symbol_node;
use ruby_prism::SymbolNode;

pub fn build_symbol_node(node: &SymbolNode<'_>, context: &mut BuildContext) -> Document {
    let location = build_node_location(&node.as_node(), context);
    layout_symbol_node(LayoutParamSymbolNode { location })
}
