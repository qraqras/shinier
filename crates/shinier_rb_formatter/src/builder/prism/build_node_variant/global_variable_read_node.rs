use crate::Document;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_node_as_location;
use ruby_prism::*;

pub fn build_global_variable_read_node(node: &GlobalVariableReadNode<'_>, context: &mut BuildContext) -> Document {
    build_node_as_location(&node.as_node(), context).unwrap()
}
