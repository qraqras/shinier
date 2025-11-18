use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::MatchRequiredNode;

pub fn build_match_required_node(node: &MatchRequiredNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    let pattern = build_node(&node.pattern(), context);
    let operator = build_location(&node.operator_loc(), context).unwrap();
    group(array(&[value, space(), operator, indent(array(&[line(), pattern]))]))
}
