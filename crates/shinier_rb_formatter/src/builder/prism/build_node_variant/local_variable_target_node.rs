use crate::Document;
use crate::builder::BuildContext;
use crate::builder::prism::build_location::build_node_as_location;
use ruby_prism::LocalVariableTargetNode;

pub fn build_local_variable_target_node(
    node: &LocalVariableTargetNode<'_>,
    ctx: &mut BuildContext,
) -> Option<Document> {
    build_node_as_location(&node.as_node(), ctx)
}
