use crate::Document;
use crate::builder::builder::*;
use crate::builder::keyword::COMMA;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ArrayPatternNode;
use ruby_prism::Node;

pub fn build_array_pattern_node(node: &ArrayPatternNode<'_>, context: &mut BuildContext) -> Document {
    let constant = node.constant();
    let requireds = node.requireds();
    let rest = node.rest();
    let posts = node.posts();
    let opening_loc = node.opening_loc();
    let closing_loc = node.closing_loc();

    // Collects all parameters
    let mut params = Vec::new();
    for required in requireds.iter() {
        params.push(required);
    }
    if let Some(rest) = rest {
        params.push(rest);
    }
    for post in posts.iter() {
        params.push(post);
    }

    // Builds parameters with separators
    let mut built_params = Vec::new();
    for (i, param) in params.into_iter().enumerate() {
        match param {
            Node::ImplicitRestNode { .. } => {}
            _ => {
                if i > 0 {
                    built_params.push(string(COMMA));
                    built_params.push(line());
                }
            }
        };
        built_params.push(build_node(&param, context));
    }

    match (&constant, &opening_loc, &closing_loc) {
        (None, None, None) => group(array(&built_params)),
        (None, Some(opening_loc), Some(closing_loc)) => group(array(&[
            build_location(opening_loc, context).unwrap(),
            indent(array(&[softline(), array(&built_params)])),
            softline(),
            build_location(closing_loc, context).unwrap(),
        ])),
        (Some(constant), Some(opening_loc), Some(closing_loc)) => group(array(&[
            build_node(constant, context),
            build_location(opening_loc, context).unwrap(),
            indent(array(&[softline(), array(&built_params)])),
            softline(),
            build_location(closing_loc, context).unwrap(),
        ])),
        _ => unreachable!(),
    }
}
