use crate::Document;
use crate::builder::BuildContext;
use crate::builder::prism::build_location::build_node_as_location;
use ruby_prism::FalseNode;

/// Builds FalseNode.
pub fn build_false_node(node: &FalseNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    build_node_as_location(&node.as_node(), ctx)
}
