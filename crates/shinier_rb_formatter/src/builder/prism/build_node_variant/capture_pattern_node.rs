use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::CapturePatternNode;

pub fn build_capture_pattern_node(node: &CapturePatternNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let value = node.value();
    let target = node.target();
    let operator_loc = node.operator_loc();

    group(array(&[
        build_node(value, ctx),
        space(),
        build_location(operator_loc, ctx),
        indent(array(&[line(), build_node(target.as_node(), ctx)])),
    ]))
}
