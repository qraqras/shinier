use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::builder_helper::*;
use ruby_prism::AssocSplatNode;

/// Builds AssocSplatNode.
///
/// Formats on a single line regardless of length, unless comments are present.
pub fn build_assoc_splat_node(node: &AssocSplatNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let value = node.value();
    let operator_loc = node.operator_loc();

    match value {
        Some(v) => {
            let line_or_none = softline_if_has_comments(operator_loc.end_offset(), v.location().start_offset(), ctx);
            group(array(&[
                build_location(operator_loc, ctx),
                line_or_none,
                build_node(v, ctx),
            ]))
        }
        None => build_location(operator_loc, ctx),
    }
}
