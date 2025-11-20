use crate::Document;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_node_as_location;
use ruby_prism::ConstantReadNode;

pub fn build_constant_read_node(node: &ConstantReadNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    build_node_as_location(&node.as_node(), ctx)
}
