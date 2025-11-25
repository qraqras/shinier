use crate::Document;
use crate::builder::prism::BuildContext;
use crate::helper::build_logical_chain::build_logical_chain;
use ruby_prism::AndNode;

/// Builds AndNode.
/// Since `and` and `&&` have different operator precedence, do not format them.
pub fn build_and_node(node: &AndNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    build_logical_chain(node.as_node(), ctx)
}
