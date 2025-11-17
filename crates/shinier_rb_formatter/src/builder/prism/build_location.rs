use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::helper::build_comments::*;
use ruby_prism::Location;
use ruby_prism::Node;

/// Checks if the location has already been processed.
fn is_processed(location: &Location, context: &mut BuildContext) -> bool {
    !context
        .processed_locations
        .insert((location.start_offset(), location.end_offset()))
}

/// Builds a Document for a given location, including leading and trailing comments.
pub fn build_location(location: &Location, context: &mut BuildContext) -> Document {
    if is_processed(location, context) {
        return none();
    }
    let leading_comments = leading_comments_l(location, context);
    let trailing_comments = trailing_comments_l(location, context);
    let content = std::str::from_utf8(location.as_slice()).unwrap_or("");
    array_opt(&[leading_comments, Some(string(content)), trailing_comments])
}

// Builds a Document for a given location with custom content, including leading and trailing comments.
pub fn build_custom_location(location: &Location, context: &mut BuildContext, content: &str) -> Document {
    if is_processed(location, context) {
        return none();
    }
    let leading_comments = leading_comments_l(location, context);
    let trailing_comments = trailing_comments_l(location, context);
    array_opt(&[leading_comments, Some(string(content)), trailing_comments])
}

/// Builds a Document for the entire source code spanned by the given node.
/// Comments and blank lines are not included here because they are already processed
/// when building the node itself in build_node().
pub fn build_node_as_location(node: &Node, context: &mut BuildContext) -> Document {
    let location = &node.location();
    if is_processed(location, context) {
        return none();
    }
    let content = std::str::from_utf8(location.as_slice()).unwrap_or("");
    string(content)
}

/// Builds a Document for the given node with custom content.
/// Comments and blank lines are not included here because they are already processed
/// when building the node itself in build_node().
pub fn build_node_as_custom_location(node: &Node, context: &mut BuildContext, content: &str) -> Document {
    let location = &node.location();
    if is_processed(location, context) {
        return none();
    }
    string(content)
}
