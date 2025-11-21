use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_custom_location;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::AssocNode;
use ruby_prism::Node;

pub fn build_assoc_node(node: &AssocNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let key = node.key();
    let value = node.value();
    let operator_loc = node.operator_loc();

    match operator_loc {
        // if operator_loc is Some, it means it's a hash rocket style (e.g., key => value)
        Some(l) => match &key.as_symbol_node() {
            // if key is a SymbolNode, we format it as hash label style
            Some(n) => group(array(&[
                build_node_as_hash_label_style(n.as_node(), ctx),
                build_custom_location(l, ctx, ":"),
                indent(array(&[line(), build_node(value, ctx)])),
            ])),
            // if key is not a SymbolNode, we format it as hash rocket style
            None => group(array(&[
                build_node(key, ctx),
                space(),
                build_location(l, ctx),
                indent(array(&[line(), build_node(value, ctx)])),
            ])),
        },
        // if operator_loc is None, it means it's a hash label style (e.g., key: value)
        None => group(array(&[
            build_node(key, ctx),
            indent(array(&[line(), build_node(value, ctx)])),
        ])),
    }
}

fn build_node_as_hash_label_style(node: Node<'_>, ctx: &mut BuildContext) -> Option<Document> {
    ctx.hash_label_style = true;
    let built = build_node(node, ctx);
    ctx.hash_label_style = false;
    built
}
