use crate::Document;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_node_as_custom_location;
use crate::builder::prism::build_location::build_node_as_location;
use ruby_prism::SymbolNode;

pub fn build_symbol_node(node: &SymbolNode<'_>, context: &mut BuildContext) -> Document {
    match context.hash_label_style {
        true => build_node_as_hash_label_style_location(&node, context),
        false => build_node_as_location(&node.as_node(), context),
    }
}

/// Build a SymbolNode as a hash label style that omits the leading colon in the location.
fn build_node_as_hash_label_style_location(node: &SymbolNode<'_>, context: &mut BuildContext) -> Document {
    let location_slice = node.location().as_slice();
    match location_slice.first() {
        Some(b':') => {
            let content = std::str::from_utf8(&location_slice[1..]).unwrap_or("");
            build_node_as_custom_location(&node.as_node(), context, content)
        }
        _ => build_node_as_location(&node.as_node(), context),
    }
}
