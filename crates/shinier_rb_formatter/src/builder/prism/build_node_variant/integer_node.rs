use crate::Document;
use crate::builder::BuildContext;
use crate::builder::prism::build_location::build_node_as_location;
use ruby_prism::IntegerNode;

pub fn build_integer_node(node: &IntegerNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    build_node_as_location(&node.as_node(), ctx)
}
