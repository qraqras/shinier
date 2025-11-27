use crate::Document;
use crate::builder::BuildContext;
use crate::builder::prism::build_location::build_node_as_location;
use ruby_prism::BlockLocalVariableNode;

/// Builds BlockLocalVariableNode.
pub fn build_block_local_variable_node(node: &BlockLocalVariableNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    build_node_as_location(&node.as_node(), ctx)
}
