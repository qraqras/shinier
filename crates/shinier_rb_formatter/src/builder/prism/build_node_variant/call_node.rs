use crate::Document;
use crate::builder::builder::*;
use crate::builder::keyword::COMMA;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::CallNode;

pub fn build_call_node(node: &CallNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let receiver = node.receiver();
    let call_operator_loc = node.call_operator_loc();
    let message_loc = node.message_loc();
    let opening_loc = node.opening_loc();
    let arguments = node.arguments();
    let closing_loc = node.closing_loc();
    let block = node.block();

    let line_or_none = match (&opening_loc, &closing_loc) {
        (Some(_), Some(_)) => softline(),
        (None, None) => None,
        _ => unreachable!(),
    };

    let separator_or_none = match (&arguments, &block) {
        (Some(_), Some(_)) => array(&[string(COMMA), line()]),
        _ => None,
    };

    // TODO: TESTING
    group(array(&[
        receiver.map(|n| build_node(n, ctx)).flatten(),
        call_operator_loc.map(|loc| build_location(loc, ctx)).flatten(),
        message_loc.map(|loc| build_location(loc, ctx)).flatten(),
        opening_loc.map(|loc| build_location(loc, ctx)).flatten(),
        match (&arguments, &block) {
            (None, None) => None,
            _ => indent(array(&[
                line_or_none.clone(),
                arguments.map(|n| build_node(n.as_node(), ctx)).flatten(),
                separator_or_none,
                block.map(|n| build_node(n, ctx)).flatten(),
            ])),
        },
        closing_loc
            .map(|loc| array(&[line_or_none.clone(), build_location(loc, ctx)]))
            .flatten(),
    ]))
}
