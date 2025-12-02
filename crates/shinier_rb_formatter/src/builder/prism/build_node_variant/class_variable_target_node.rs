use crate::Document;
use crate::builder::BuildContext;
use crate::builder::prism::build_location::build_node_as_location;
use ruby_prism::ClassVariableTargetNode;

/// Builds ClassVariableTargetNode.
pub fn build_class_variable_target_node(
    node: &ClassVariableTargetNode<'_>,
    ctx: &mut BuildContext,
) -> Option<Document> {
    build_node_as_location(&node.as_node(), ctx)
}
