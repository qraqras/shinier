use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::CallNode;
use ruby_prism::Node;

/// Builds CallNode for method calls.
///
/// ## Parentheses handling:
/// - Preserves existing parentheses: `foo(1, 2)` → `foo(1, 2)`
/// - Without parentheses: `foo 1, 2` → adds conditional parentheses on line break
/// - Empty parentheses: `foo()` → `foo` (omits empty parentheses)
///
/// ## Block handling:
/// - Block argument (`&block`): formatted inside parentheses with arguments
/// - Block literal (`{ }` / `do...end`): formatted after parentheses
pub fn build_call_node(node: &CallNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let receiver = node.receiver();
    let call_operator_loc = node.call_operator_loc();
    let message_loc = node.message_loc();
    let opening_loc = node.opening_loc();
    let arguments = node.arguments();
    let closing_loc = node.closing_loc();
    let block = node.block();

    // Separates block argument (&block) from block literal ({ } / do...end).
    // Block argument is formatted inside parentheses, block literal is formatted after.
    let (block_argument, block_literal) = match block {
        Some(block) => match block {
            Node::BlockArgumentNode { .. } => (Some(block), None),
            Node::BlockNode { .. } => (None, Some(block)),
            _ => unreachable!("Unexpected node type for block"),
        },
        None => (None, None),
    };

    // Builds combined arguments (regular arguments + block argument).
    let args_content = match (arguments, block_argument) {
        (Some(a), Some(b)) => array(&[build_node(a.as_node(), ctx), comma(), line(), build_node(b, ctx)]),
        (Some(a), None) => build_node(a.as_node(), ctx),
        (None, Some(b)) => build_node(b, ctx),
        (None, None) => None,
    };

    // Wraps arguments with parentheses (or adds conditional parentheses).
    let args_doc = match (opening_loc, args_content, closing_loc) {
        (Some(opening), Some(content), Some(closing)) => group(array(&[
            build_location(opening, ctx),
            indent(array(&[softline(), Some(content)])),
            softline(),
            build_location(closing, ctx),
        ])),
        (Some(_), None, Some(_)) => None,
        (None, Some(content), None) => group(array(&[
            if_break(string("("), None, None),
            indent(array(&[line(), Some(content)])),
            softline(),
            if_break(string(")"), None, None),
        ])),
        (None, None, None) => None,
        _ => unreachable!(),
    };

    group(array(&[
        receiver.map(|n| build_node(n, ctx)).flatten(),
        call_operator_loc.map(|loc| build_location(loc, ctx)).flatten(),
        message_loc.map(|loc| build_location(loc, ctx)).flatten(),
        args_doc,
        block_literal.map(|n| array(&[space(), build_node(n, ctx)])).flatten(),
    ]))
}
