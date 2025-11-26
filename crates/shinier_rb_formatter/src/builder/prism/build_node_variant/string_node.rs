use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use ruby_prism::StringNode;

pub fn build_string_node(node: &StringNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    // TODO: Use single quotes if possible
    let opening_loc = node.opening_loc();
    let content_loc = node.content_loc();
    let closing_loc = node.closing_loc();
    array(&[
        opening_loc.map(|loc| build_location(loc, ctx)).flatten(),
        build_location(content_loc, ctx),
        closing_loc.map(|loc| build_location(loc, ctx)).flatten(),
    ])
}
