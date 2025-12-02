use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::builder_helper::*;
use ruby_prism::BlockArgumentNode;

/// Builds BlockArgumentNode.
pub fn build_block_argument_node(node: &BlockArgumentNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let expression = node.expression();
    let operator_loc = node.operator_loc();

    match expression {
        Some(e) => {
            let line_or_none = softline_if_has_comments(operator_loc.end_offset(), e.location().start_offset(), ctx);
            group(array(&[
                build_location(operator_loc, ctx),
                indent(array(&[line_or_none, build_node(e, ctx)])),
            ]))
        }
        None => build_location(operator_loc, ctx),
    }
}
