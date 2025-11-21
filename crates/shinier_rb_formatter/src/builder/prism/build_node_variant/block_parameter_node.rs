use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use ruby_prism::BlockParameterNode;

pub fn build_block_parameter_node(node: &BlockParameterNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let name_loc = node.name_loc();
    let oprator_loc = node.operator_loc();

    group(array(&[
        build_location(&oprator_loc, ctx),
        name_loc
            .map(|n| array(&[softline(), build_location(&n, ctx)]))
            .flatten(),
    ]))
}
