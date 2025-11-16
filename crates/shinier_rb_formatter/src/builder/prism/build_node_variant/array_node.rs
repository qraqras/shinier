use crate::Document;
use crate::builder::builder::*;
use crate::builder::keyword::COMMA;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ArrayNode;
use ruby_prism::Location;

pub fn build_array_node(node: &ArrayNode<'_>, context: &mut BuildContext) -> Document {
    let elements = node.elements();
    let opening_loc = node.opening_loc();
    let closing_loc = node.closing_loc();
    let sparator = match is_percent_array(&opening_loc) {
        true => line(),
        false => array(&[string(COMMA), line()]),
    };
    let mut parts = Vec::new();
    for (i, element) in elements.iter().enumerate() {
        if i > 0 {
            parts.push(sparator.clone());
        }
        parts.push(build_node(&element, context));
    }
    group(array_opt(&[
        opening_loc.as_ref().map(|loc| build_location(loc, context)),
        Some(indent(array(&[softline(), array(&parts)]))),
        Some(softline()),
        closing_loc.as_ref().map(|loc| build_location(loc, context)),
    ]))
}

fn is_percent_array(opening_loc: &Option<Location>) -> bool {
    opening_loc
        .as_ref()
        .and_then(|loc| loc.as_slice().first())
        .map(|&f| f == b'%')
        .unwrap_or(false)
}
