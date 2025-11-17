use crate::Document;
use crate::builder::COMMA;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::HashNode;

pub fn build_hash_node(node: &HashNode<'_>, context: &mut BuildContext) -> Document {
    let opening_loc = node.opening_loc();
    let elements = node.elements();
    let closing_loc = node.closing_loc();

    let mut parts = Vec::new();
    for (i, element) in elements.iter().enumerate() {
        if i > 0 {
            parts.push(string(COMMA));
            parts.push(line());
        }
        parts.push(build_node(&element, context));
    }

    // If there are no elements, use softline to avoid unnecessary spaces inside the braces.
    let line_or_softline = match parts.is_empty() {
        true => softline(),
        false => line(),
    };

    group(array(&[
        build_location(&opening_loc, context),
        indent(array(&[line_or_softline.clone(), array(&parts)])),
        line_or_softline.clone(),
        build_location(&closing_loc, context),
    ]))
}
