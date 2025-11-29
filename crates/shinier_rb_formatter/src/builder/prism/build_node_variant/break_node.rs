use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::BreakNode;

/// Builds BreakNode.
pub fn build_break_node(node: &BreakNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let arguments = node.arguments();
    let keyword_loc = node.keyword_loc();
    group(array(&[
        build_location(keyword_loc, ctx),
        space(),
        indent(arguments.map(|a| build_node(a.as_node(), ctx)).flatten()),
    ]))
}
