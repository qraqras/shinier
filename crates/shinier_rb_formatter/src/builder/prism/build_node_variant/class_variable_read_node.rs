use crate::Document;
use crate::builder::BuildContext;
use crate::builder::prism::build_location::build_node_as_location;
use ruby_prism::ClassVariableReadNode;

/// Builds ClassVariableReadNode.
pub fn build_class_variable_read_node(node: &ClassVariableReadNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    build_node_as_location(&node.as_node(), ctx)
}
