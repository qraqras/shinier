use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::builder_helper::*;
use ruby_prism::AliasGlobalVariableNode;

/// Builds AliasGlobalVariableNode.
///
/// Formats on a single line regardless of length, unless comments are present.
pub fn build_alias_global_variable_node(
    node: &AliasGlobalVariableNode<'_>,
    ctx: &mut BuildContext,
) -> Option<Document> {
    let keyword_loc = node.keyword_loc();
    let new_name = node.new_name();
    let old_name = node.old_name();

    let break_or_space = line_if_has_comments(keyword_loc.end_offset(), new_name.location().start_offset(), ctx);
    group(array(&[
        build_location(keyword_loc, ctx),
        indent(array(&[
            break_or_space,
            build_node(new_name, ctx),
            space(),
            build_node(old_name, ctx),
        ])),
    ]))
}
