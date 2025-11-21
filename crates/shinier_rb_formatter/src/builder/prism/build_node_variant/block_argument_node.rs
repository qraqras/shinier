use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::BlockArgumentNode;

pub fn build_block_argument_node(node: &BlockArgumentNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let expression = node.expression();
    let operator_loc = node.operator_loc();

    group(array(&[
        build_location(operator_loc, ctx),
        expression.map(|e| array(&[softline(), build_node(e, ctx)])).flatten(),
    ]))
}
