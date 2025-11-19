use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::helper::build_comments::*;
use ruby_prism::Location;
use ruby_prism::Node;

/// Builds a Document for a given location, including leading and trailing comments.
pub fn build_location(location: &Location, context: &mut BuildContext) -> Option<Document> {
    let content = std::str::from_utf8(location.as_slice()).unwrap_or("");
    build_custom_location(location, context, content)
}

// Builds a Document for a given location with custom content, including leading and trailing comments.
pub fn build_custom_location(location: &Location, context: &mut BuildContext, content: &str) -> Option<Document> {
    if is_processed(location, context) {
        return None;
    }
    let remaining_comments = context.remaining_comments.take();
    let leading_comments = context
        .comment_store
        .pop_leading(location.start_offset(), location.end_offset());
    let trailing_comments = context
        .comment_store
        .pop_trailing(location.start_offset(), location.end_offset());
    let dangling_comments = context
        .comment_store
        .pop_dangling(location.start_offset(), location.end_offset());
    // merge remaining comments into leading comments
    let leading_comments = match (remaining_comments, leading_comments) {
        (Some(mut remaining), Some(leading)) => {
            remaining.extend(leading);
            Some(remaining)
        }
        (Some(remaining), None) => Some(remaining),
        (None, Some(leading)) => Some(leading),
        (None, None) => None,
    };
    Some(array_opt(&[
        leading_comments
            .map(|c| build_comments_as_leading(Some(c), context))
            .flatten(),
        Some(string(content)),
        trailing_comments
            .map(|c| build_comments_as_trailing(Some(c), context))
            .flatten(),
        dangling_comments
            .map(|d| build_comments_as_dangling(Some(d), context))
            .flatten(),
    ]))
}

/// Builds a Document for the entire source code spanned by the given node.
/// Comments and blank lines are not included here because they are already processed
/// when building the node itself in build_node().
pub fn build_node_as_location(node: &Node, context: &mut BuildContext) -> Option<Document> {
    let location = &node.location();
    let content = std::str::from_utf8(location.as_slice()).unwrap_or("");
    build_node_as_custom_location(node, context, content)
}

/// Builds a Document for the given node with custom content.
/// Comments and blank lines are not included here because they are already processed
/// when building the node itself in build_node().
pub fn build_node_as_custom_location(node: &Node, context: &mut BuildContext, content: &str) -> Option<Document> {
    let location = &node.location();
    if is_processed(location, context) {
        return None;
    }
    Some(string(content))
}

/// Checks if the location has already been processed.
fn is_processed(location: &Location, context: &mut BuildContext) -> bool {
    !context
        .processed_locations
        .insert((location.start_offset(), location.end_offset()))
}
