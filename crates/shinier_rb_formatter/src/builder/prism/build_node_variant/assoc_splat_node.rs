use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::AssocSplatNode;

pub fn build_assoc_splat_node(node: &AssocSplatNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let value = node.value();
    let operator_loc = node.operator_loc();
    group(array(&[
        build_location(operator_loc, ctx),
        value.map(|v| build_node(v, ctx)).flatten(),
    ]))
}
