use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_custom_location;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::AssocNode;
use ruby_prism::Node;

pub fn build_assoc_node(node: &AssocNode<'_>, context: &mut BuildContext) -> Document {
    let key = node.key();
    let value = node.value();
    let operator_loc = node.operator_loc();

    match operator_loc {
        // if operator_loc is Some, it means it's a hash rocket style (e.g., key => value)
        Some(l) => match &key.as_symbol_node() {
            // if key is a SymbolNode, we format it as hash label style
            Some(n) => group(array(&[
                build_node_as_hash_label_style(&n.as_node(), context),
                build_custom_location(&l, context, ":").unwrap(),
                indent(array(&[line(), build_node(&value, context)])),
            ])),
            // if key is not a SymbolNode, we format it as hash rocket style
            None => group(array(&[
                build_node(&key, context),
                space(),
                build_location(&l, context).unwrap(),
                indent(array(&[line(), build_node(&value, context)])),
            ])),
        },
        // if operator_loc is None, it means it's a hash label style (e.g., key: value)
        None => group(array(&[
            build_node(&key, context),
            indent(array(&[line(), build_node(&value, context)])),
        ])),
    }
}

fn build_node_as_hash_label_style(node: &Node<'_>, context: &mut BuildContext) -> Document {
    context.hash_label_style = true;
    let built = build_node(&node, context);
    context.hash_label_style = false;
    built
}
