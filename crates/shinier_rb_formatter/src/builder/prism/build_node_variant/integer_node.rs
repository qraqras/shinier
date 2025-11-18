use crate::Document;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_node_as_location;
use ruby_prism::IntegerNode;

pub fn build_integer_node(node: &IntegerNode<'_>, context: &mut BuildContext) -> Document {
    build_node_as_location(&node.as_node(), context).unwrap()
}
