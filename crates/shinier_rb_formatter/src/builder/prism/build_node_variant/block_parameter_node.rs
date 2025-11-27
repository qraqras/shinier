use crate::Document;
use crate::builder::builder::*;
use crate::builder::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::builder_helper::*;
use ruby_prism::BlockParameterNode;

/// Builds BlockParameterNode.
///
/// Formats on a single line regardless of length, unless comments are present.
pub fn build_block_parameter_node(node: &BlockParameterNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let name_loc = node.name_loc();
    let oprator_loc = node.operator_loc();

    match name_loc {
        Some(n) => {
            let line_or_none = softline_if_has_comments(oprator_loc.end_offset(), n.start_offset(), ctx);
            group(array(&[
                build_location(oprator_loc, ctx),
                line_or_none,
                build_location(n, ctx),
            ]))
        }
        None => build_location(oprator_loc, ctx),
    }
}
