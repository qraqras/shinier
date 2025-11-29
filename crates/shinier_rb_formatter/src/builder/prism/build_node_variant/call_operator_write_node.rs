use crate::Document;
use crate::builder::builder::*;
use crate::builder::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::CallOperatorWriteNode;

/// Builds CallOperatorWriteNode.
pub fn build_call_operator_write_node(node: &CallOperatorWriteNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let receiver = node.receiver();
    let call_operator_loc = node.call_operator_loc();
    let message_loc = node.message_loc();
    let binary_operator_loc = node.binary_operator_loc();
    let value = node.value();

    group(array(&[
        receiver.map(|r| build_node(r, ctx)).flatten(),
        call_operator_loc.map(|c| build_location(c, ctx)).flatten(),
        message_loc.map(|m| build_location(m, ctx)).flatten(),
        space(),
        build_location(binary_operator_loc, ctx),
        indent(array(&[line(), build_node(value, ctx)])),
    ]))
}
