use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::helper::location_helper::*;
use ruby_prism::ArrayNode;

/// Builds ArrayNode.
///
/// Formats arrays in two styles:
/// - Basic Array Style: `[a, b, c]`
/// - Percent Array Style: `%w[a b c]`
///
/// Items are separated by commas, except for percent arrays.
pub fn build_array_node(node: &ArrayNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let elements = node.elements();
    let opening_loc = node.opening_loc();
    let closing_loc = node.closing_loc();

    // Checks if the array is a percent array.
    let is_percent_array = _is_percent_array(node);

    // Builds each item in the array with appropriate separators.
    let mut parts = Vec::new();
    for (i, element) in elements.iter().enumerate() {
        if i > 0 {
            if is_percent_array {
                parts.push(space());
            } else {
                parts.push(comma());
                parts.push(line());
            }
        }
        parts.push(build_node(element, ctx));
    }

    group(array(&[
        opening_loc.map(|l| build_location(l, ctx)).flatten(),
        indent(array(&[softline(), array(&parts)])),
        softline(),
        closing_loc.map(|l| build_location(l, ctx)).flatten(),
    ]))
}

/// Checks if the array is a percent array (e.g., `%w[]`, `%i[]`, etc.)
fn _is_percent_array(node: &ArrayNode<'_>) -> bool {
    let opening_loc = node.opening_loc();
    match opening_loc {
        Some(loc) => starts_with(&loc, "%"),
        None => false,
    }
}
