use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::BeginNode;

pub fn build_begin_node(node: &BeginNode<'_>, context: &mut BuildContext) -> Document {
    let begin_keyword_loc = node.begin_keyword_loc();
    let statements = node.statements();
    let rescue_clause = node.rescue_clause();
    let else_clause = node.else_clause();
    let ensure_clause = node.ensure_clause();
    let end_keyword_loc = node.end_keyword_loc();

    group(array_opt(&[
        begin_keyword_loc.map(|loc| build_location(&loc, context).unwrap_or(none())),
        statements.map(|n| indent(array(&[hardline(), build_node(&n.as_node(), context)]))),
        rescue_clause.map(|n| array(&[hardline(), build_node(&n.as_node(), context)])),
        else_clause.map(|n| array(&[hardline(), build_node(&n.as_node(), context)])),
        ensure_clause.map(|n| array(&[hardline(), build_node(&n.as_node(), context)])),
        end_keyword_loc.and_then(|loc| build_location(&loc, context).map(|e| array(&[hardline(), e]))),
    ]))
}
