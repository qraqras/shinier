use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::SplatNode;

pub fn build_splat_node(node: &SplatNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let operator_loc = node.operator_loc();
    let expression = node.expression();
    group(array(&[
        build_location(&operator_loc, ctx),
        expression.as_ref().map(|n| build_node(&n, ctx)).flatten(),
    ]))
}
