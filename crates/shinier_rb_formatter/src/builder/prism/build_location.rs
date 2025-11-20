use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_main::build_main;
use crate::comments::Target;
use ruby_prism::Location;
use ruby_prism::Node;

/// Internal function to build a location with optional content
fn _location_builder(loc: &Location, _ctx: &mut BuildContext, custom_content: &Option<&str>) -> Option<Document> {
    let content = match custom_content {
        Some(c) => c,
        None => std::str::from_utf8(loc.as_slice()).unwrap_or(""),
    };
    string(content)
}

/// Checks if the location has already been processed.
fn _is_processed(loc: &Location, ctx: &mut BuildContext) -> bool {
    !ctx.processed_locations.insert((loc.start_offset(), loc.end_offset()))
}

/// Internal function to build a location with optional custom content.
fn _build_location(loc: &Location, ctx: &mut BuildContext, content: Option<&str>) -> Option<Document> {
    if _is_processed(loc, ctx) {
        return None;
    }
    build_main(_location_builder, loc, &content, ctx, &Target::from_location(loc))
}

/// Internal function to build a node as a location with optional custom content.
fn _build_node_as_location(node: &Node, ctx: &mut BuildContext, content: Option<&str>) -> Option<Document> {
    let location = &node.location();
    if _is_processed(location, ctx) {
        return None;
    }
    let loc = &node.location();
    _location_builder(loc, ctx, &content)
}

/// Builds a Document for a given location.
pub fn build_location(loc: &Location, ctx: &mut BuildContext) -> Option<Document> {
    _build_location(loc, ctx, None)
}

/// Builds a Document for a given location with custom content.
pub fn build_custom_location(loc: &Location, ctx: &mut BuildContext, content: &str) -> Option<Document> {
    _build_location(loc, ctx, Some(content))
}

/// Builds a Document for the given node.
/// Comments and blank lines are not included here because they are already processed
/// when building the node itself in build_node().
pub fn build_node_as_location(node: &Node, ctx: &mut BuildContext) -> Option<Document> {
    _build_node_as_location(node, ctx, None)
}

/// Builds a Document for the given node with custom content.
/// Comments and blank lines are not included here because they are already processed
/// when building the node itself in build_node().
pub fn build_node_as_custom_location(node: &Node, ctx: &mut BuildContext, content: &str) -> Option<Document> {
    _build_node_as_location(node, ctx, Some(content))
}
