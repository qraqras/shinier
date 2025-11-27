use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::MultiWriteNode;
use ruby_prism::Node;

/// Builds MultiWriteNode.
pub fn build_multi_write_node(node: &MultiWriteNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let lefts = node.lefts();
    let rest = node.rest();
    let rights = node.rights();
    let lparen_loc = node.lparen_loc();
    let rparen_loc = node.rparen_loc();
    let operator_loc = node.operator_loc();
    let value = node.value();

    // Collects parameters.
    let mut params = Vec::new();
    params.extend(lefts.iter());
    rest.map(|r| params.push(r));
    params.extend(rights.iter());

    // Builds documents for parameters.
    let mut parameters_doc = Vec::new();
    for (i, param) in params.into_iter().enumerate() {
        if i > 0 && !matches!(param, Node::ImplicitRestNode { .. }) {
            parameters_doc.push(comma());
            parameters_doc.push(line());
        }
        parameters_doc.push(build_node(param, ctx));
    }

    match (lparen_loc, rparen_loc) {
        (Some(lparen), Some(rparen)) => group(array(&[
            build_location(lparen, ctx),
            indent(array(&[softline(), array(&parameters_doc)])),
            softline(),
            build_location(rparen, ctx),
            space(),
            build_location(operator_loc, ctx),
            line(),
            build_node(value, ctx),
        ])),
        (None, None) => group(array(&[
            array(&parameters_doc),
            space(),
            build_location(operator_loc, ctx),
            line(),
            build_node(value, ctx),
        ])),
        _ => unreachable!(),
    }
}
