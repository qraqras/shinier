use crate::Document;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_node_as_location;
use ruby_prism::ImplicitRestNode;

pub fn build_implicit_rest_node(node: &ImplicitRestNode<'_>, context: &mut BuildContext) -> Document {
    build_node_as_location(&node.as_node(), context)
}
