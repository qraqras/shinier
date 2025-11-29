use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ArrayPatternNode;
use ruby_prism::Node;
use ruby_prism::NodeList;

/// Builds ArrayPatternNode.
///
/// Note that the trailing comma is part of the syntax for matching the remaining elements.
pub fn build_array_pattern_node(node: &ArrayPatternNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let constant = node.constant();
    let requireds = node.requireds();
    let rest = node.rest();
    let posts = node.posts();
    let opening_loc = node.opening_loc();
    let closing_loc = node.closing_loc();

    let params = _collect_parameters(&requireds, rest, &posts);
    let docs = _build_parameters(params, ctx);

    match (opening_loc, closing_loc) {
        (None, None) => group(array(&docs)),
        (Some(opening_loc), Some(closing_loc)) => group(array(&[
            constant.map(|c| build_node(c, ctx)).flatten(),
            build_location(opening_loc, ctx),
            indent(array(&[softline(), array(&docs)])),
            softline(),
            build_location(closing_loc, ctx),
        ])),
        _ => unreachable!(),
    }
}

/// Collects all parameters into a single vector.
fn _collect_parameters<'sh>(
    requireds: &NodeList<'sh>,
    rest: Option<Node<'sh>>,
    posts: &NodeList<'sh>,
) -> Vec<Node<'sh>> {
    let capacity = requireds.iter().count() + rest.is_some() as usize + posts.iter().count();
    let mut parameters = Vec::with_capacity(capacity);
    parameters.extend(requireds.iter());
    rest.map(|r| parameters.push(r));
    parameters.extend(posts.iter());
    parameters
}

/// Builds documents for parameters with proper separators.
fn _build_parameters(parameters: Vec<Node<'_>>, ctx: &mut BuildContext) -> Vec<Option<Document>> {
    let mut docs = Vec::with_capacity(parameters.len() * 3);
    for (i, param) in parameters.into_iter().enumerate() {
        // ImplicitRestNode should not be followed by a comma.
        if i > 0 && !matches!(param, Node::ImplicitRestNode { .. }) {
            docs.push(comma());
            docs.push(line());
        }
        docs.push(build_node(param, ctx));
    }
    docs
}
