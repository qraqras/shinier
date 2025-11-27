use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::*;

/// Builds BlockNode.
///
/// Parameters are kept on a single line when possible,
/// while the body breaks independently based on its content.
///
/// The original block style (`{ }` or `do...end`) is preserved without conversion.
/// TODO: Consider adding a feature to automatically convert block styles
pub fn build_block_node(node: &BlockNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let parameters = node.parameters();
    let body = node.body();
    let opening_loc = node.opening_loc();
    let closing_loc = node.closing_loc();

    group(array(&[
        build_location(opening_loc, ctx),
        indent(match (parameters, body) {
            (Some(p), Some(b)) => array(&[group(array(&[line(), build_node(p, ctx)])), line(), build_node(b, ctx)]),
            (Some(p), None) => group(array(&[line(), build_node(p, ctx)])),
            (None, Some(b)) => group(array(&[line(), build_node(b, ctx)])),
            (None, None) => None,
        }),
        line(),
        build_location(closing_loc, ctx),
    ]))
}
