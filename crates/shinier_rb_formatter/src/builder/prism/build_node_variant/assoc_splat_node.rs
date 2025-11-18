use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::AssocSplatNode;

pub fn build_assoc_splat_node(node: &AssocSplatNode<'_>, context: &mut BuildContext) -> Document {
    let value = node.value();
    let operator_loc = node.operator_loc();
    group(array_opt(&[
        build_location(&operator_loc, context),
        value.map(|n| build_node(&n, context)),
    ]))
}
