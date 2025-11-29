use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::MatchPredicateNode;

pub fn build_match_predicate_node(node: &MatchPredicateNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let value = build_node(node.value(), ctx);
    let pattern = build_node(node.pattern(), ctx);
    let operator = build_location(node.operator_loc(), ctx);
    group(array(&[value, space(), operator, indent(array(&[line(), pattern]))]))
}
