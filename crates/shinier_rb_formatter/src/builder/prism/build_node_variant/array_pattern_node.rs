use crate::Document;
use crate::builder::builder::*;
use crate::builder::keyword::COMMA;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ArrayPatternNode;
use ruby_prism::Node;

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

    // Collects all parameters
    let mut parameters = Vec::new();
    for required in requireds.iter() {
        parameters.push(required);
    }
    if let Some(rest) = rest {
        parameters.push(rest);
    }
    for post in posts.iter() {
        parameters.push(post);
    }

    // Builds parameters with separators
    let mut parameter_docs = Vec::new();
    for (i, param) in parameters.into_iter().enumerate() {
        match param {
            Node::ImplicitRestNode { .. } => {}
            _ => {
                if i > 0 {
                    parameter_docs.push(comma());
                    parameter_docs.push(line());
                }
            }
        };
        parameter_docs.push(build_node(param, ctx));
    }

    match (opening_loc, closing_loc) {
        (None, None) => group(array(&parameter_docs)),
        (Some(opening_loc), Some(closing_loc)) => group(array(&[
            constant.map(|c| build_node(c, ctx)).flatten(),
            build_location(opening_loc, ctx),
            indent(array(&[softline(), array(&parameter_docs)])),
            softline(),
            build_location(closing_loc, ctx),
        ])),
        _ => unreachable!(),
    }
}
