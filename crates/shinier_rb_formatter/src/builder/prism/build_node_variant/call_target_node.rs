use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::CallTargetNode;

/// Builds CallTargetNode.
pub fn build_call_target_node(node: &CallTargetNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let receiver = node.receiver();
    let call_operator_loc = node.call_operator_loc();
    let message_loc = node.message_loc();

    group(array(&[
        build_node(receiver, ctx),
        build_location(call_operator_loc, ctx),
        build_location(message_loc, ctx),
    ]))
}
