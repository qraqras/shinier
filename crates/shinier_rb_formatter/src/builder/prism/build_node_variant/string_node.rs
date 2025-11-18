use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use ruby_prism::StringNode;

pub fn build_string_node(node: &StringNode<'_>, context: &mut BuildContext) -> Document {
    // TODO: Use single quotes if possible
    let opening_loc = node.opening_loc();
    let content_loc = node.content_loc();
    let closing_loc = node.closing_loc();
    array_opt(&[
        opening_loc.as_ref().map(|loc| build_location(loc, context).unwrap()),
        Some(build_location(&content_loc, context).unwrap()),
        closing_loc.as_ref().map(|loc| build_location(loc, context).unwrap()),
    ])
}
