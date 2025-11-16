use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_node_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::CallNode;

pub fn build_call_node(node: &CallNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = match &node.receiver() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let block = match node.block() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    // TODO: not yet implemented method call formatting
    build_node_location(&node.as_node(), context)
}
