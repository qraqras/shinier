use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::CallAndWriteNode;

/// Builds CallAndWriteNode.
pub fn build_call_and_write_node(node: &CallAndWriteNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let receiver = node.receiver();
    let call_operator_loc = node.call_operator_loc();
    let message_loc = node.message_loc();
    let operator_loc = node.operator_loc();
    let value = node.value();

    group(array(&[
        receiver.map(|r| build_node(r, ctx)).flatten(),
        call_operator_loc.map(|c| build_location(c, ctx)).flatten(),
        message_loc.map(|m| build_location(m, ctx)).flatten(),
        space(),
        build_location(operator_loc, ctx),
        indent(array(&[line(), build_node(value, ctx)])),
    ]))
}
