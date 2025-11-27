use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::LambdaNode;

pub fn build_lambda_node(node: &LambdaNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let operator_loc = node.operator_loc();
    let opening_loc = node.opening_loc();
    let closing_loc = node.closing_loc();
    let parameters = node.parameters();
    let body = node.body();

    group(array(&[
        build_location(operator_loc, ctx),
        parameters.map(|p| build_node(p, ctx)).flatten(),
        space(),
        build_location(opening_loc, ctx),
        indent(body.map(|n| array(&[softline(), build_node(n, ctx)])).flatten()),
        softline(),
        build_location(closing_loc, ctx),
    ]))
}
