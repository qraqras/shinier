use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::OptionalParameterNode;

pub fn build_optional_parameter_node(node: &OptionalParameterNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let name_loc = node.name_loc();
    let operator_loc = node.operator_loc();
    let value = node.value();

    group(array(&[
        build_location(name_loc, ctx),
        space(),
        build_location(operator_loc, ctx),
        line(),
        build_node(value, ctx),
    ]))
}
