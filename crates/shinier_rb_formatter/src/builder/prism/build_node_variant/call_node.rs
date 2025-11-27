use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::builder_helper::*;
use crate::builder::prism::helper::location_helper::*;
use ruby_prism::CallNode;
use ruby_prism::Node;

/// Builds CallNode for method calls.
///
/// ## Style handling:
/// - Unary: `-foo` , `!bar`
///   - Only `not` operator requires a space after it
/// - Binary: `foo + bar`, `a && b`
/// - Call: `foo.bar`, `foo&.bar`, `foo`, `foo(bar)`, `foo bar`
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
    let _message_loc = node.message_loc();
    let _opening_loc = node.opening_loc();
    let arguments = node.arguments();
    let _closing_loc = node.closing_loc();
    let _block = node.block();

    // Determines the call style based on node structure:
    // - Unary:  receiver exists, no call operator, no arguments  (+foo, !bar)
    // - Binary: receiver exists, no call operator, has arguments (foo + bar)
    // - Call:   has call operator OR no receiver                 (foo.bar, puts foo)
    match receiver {
        Some(_) => match call_operator_loc {
            Some(_) => build_call(node, ctx),
            None => match arguments {
                Some(_) => build_binary(node, ctx),
                None => build_unary(node, ctx),
            },
        },
        None => build_call(node, ctx),
    }
}

/// Builds CallNode as unary style.
///
/// Formats on a single line regardless of length, unless comments are present.
fn build_unary(node: &CallNode, ctx: &mut BuildContext) -> Option<Document> {
    let receiver = node.receiver();
    let call_operator_loc = node.call_operator_loc();
    let message_loc = node.message_loc();
    let opening_loc = node.opening_loc();
    let arguments = node.arguments();
    let closing_loc = node.closing_loc();
    let block = node.block();

    debug_assert!(receiver.is_some());
    debug_assert!(call_operator_loc.is_none());
    debug_assert!(message_loc.is_some());
    debug_assert!(opening_loc.is_none());
    debug_assert!(arguments.is_none());
    debug_assert!(closing_loc.is_none());
    debug_assert!(block.is_none());

    let message_loc_end_offset = message_loc.as_ref().unwrap().end_offset();
    let receiver_start_offset = receiver.as_ref().unwrap().location().start_offset();

    // The `not` operator should have a space after it. Other unary operators should not.
    let break_if_has_comment = match equals(message_loc.as_ref().unwrap(), "not") {
        true => line_if_has_comments(message_loc_end_offset, receiver_start_offset, ctx),
        false => softline_if_has_comments(message_loc_end_offset, receiver_start_offset, ctx),
    };

    group(array(&[
        build_location(message_loc.unwrap(), ctx),
        indent(array(&[break_if_has_comment, build_node(receiver.unwrap(), ctx)])),
    ]))
}

/// Builds CallNode as binary style.
fn build_binary(node: &CallNode, ctx: &mut BuildContext) -> Option<Document> {
    let receiver = node.receiver();
    let call_operator_loc = node.call_operator_loc();
    let message_loc = node.message_loc();
    let opening_loc = node.opening_loc();
    let arguments = node.arguments();
    let closing_loc = node.closing_loc();
    let block = node.block();

    debug_assert!(receiver.is_some());
    debug_assert!(call_operator_loc.is_none());
    debug_assert!(message_loc.is_some());
    debug_assert!(opening_loc.is_none());
    debug_assert!(arguments.is_some());
    debug_assert!(closing_loc.is_none());
    debug_assert!(block.is_none());

    group(array(&[
        build_node(receiver.unwrap(), ctx),
        space(),
        build_location(message_loc.unwrap(), ctx),
        indent(array(&[line(), build_node(arguments.unwrap().as_node(), ctx)])),
    ]))
}

/// Builds CallNode as call style.
fn build_call(node: &CallNode, ctx: &mut BuildContext) -> Option<Document> {
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
        receiver.map(|r| build_node(r, ctx)).flatten(),
        call_operator_loc.map(|c| build_location(c, ctx)).flatten(),
        message_loc.map(|m| build_location(m, ctx)).flatten(),
        args_doc,
        block_literal.map(|b| array(&[space(), build_node(b, ctx)])).flatten(),
    ]))
}
