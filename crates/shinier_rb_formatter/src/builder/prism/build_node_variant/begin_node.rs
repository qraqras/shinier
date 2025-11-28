use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_comments::build_dangling;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::BeginNode;

/// Builds BeginNode.
pub fn build_begin_node(node: &BeginNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let begin_keyword_loc = node.begin_keyword_loc();
    let statements = node.statements();
    let rescue_clause = node.rescue_clause();
    let else_clause = node.else_clause();
    let ensure_clause = node.ensure_clause();
    let end_keyword_loc = node.end_keyword_loc();

    group(array(&[
        begin_keyword_loc.map(|loc| build_location(loc, ctx)).flatten(),
        statements
            .map(|n| indent(array(&[hardline(), build_node(n.as_node(), ctx)])))
            .flatten(),
        //
        build_dangling(&node.as_node(), ctx),
        //
        rescue_clause
            .map(|n| array(&[hardline(), build_node(n.as_node(), ctx)]))
            .flatten(),
        else_clause
            .map(|n| array(&[hardline(), build_node(n.as_node(), ctx)]))
            .flatten(),
        ensure_clause
            .map(|n| array(&[hardline(), build_node(n.as_node(), ctx)]))
            .flatten(),
        end_keyword_loc
            .and_then(|loc| build_location(loc, ctx).map(|e| array(&[hardline(), Some(e)])))
            .flatten(),
    ]))
}
