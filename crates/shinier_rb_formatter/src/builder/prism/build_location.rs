use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::helper::build_comments::*;
use ruby_prism::Location;
use ruby_prism::Node;
use std::io::Read;

/// Builds a Document for a given location, including leading and trailing comments.
pub fn build_location(location: &Location, context: &mut BuildContext) -> Document {
    let leading_comments = leading_comments_l(location, context);
    let trailing_comments = trailing_comments_l(location, context);
    let mut buf = String::new();
    let _ = location.as_slice().read_to_string(&mut buf);
    array_opt(&[leading_comments, Some(string(buf)), trailing_comments])
}

// Builds a Document for a given location with custom content, including leading and trailing comments.
pub fn build_custom_location(
    location: &Location,
    context: &mut BuildContext,
    content: &str,
) -> Document {
    let leading_comments = leading_comments_l(location, context);
    let trailing_comments = trailing_comments_l(location, context);
    array_opt(&[leading_comments, Some(string(content)), trailing_comments])
}

/// Builds a Document for the entire source code spanned by the given node.
/// Comments and blank lines are not included here because they are already processed
/// when building the node itself in build_node().
pub fn build_node_as_location(node: &Node, _context: &mut BuildContext) -> Document {
    let location = &node.location();
    let mut buf = String::new();
    let _ = location.as_slice().read_to_string(&mut buf);
    string(buf)
}
