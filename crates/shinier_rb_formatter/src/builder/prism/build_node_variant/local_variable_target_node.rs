use crate::Document;
use crate::build_location::build_node_as_location;
use crate::builder::prism::BuildContext;
use ruby_prism::LocalVariableTargetNode;

pub fn build_local_variable_target_node(node: &LocalVariableTargetNode<'_>, context: &mut BuildContext) -> Document {
    build_node_as_location(&node.as_node(), context)
}
