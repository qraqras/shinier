use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::Node;

/// Builds Logical Chain (e.g., AndNode, OrNode).
pub fn build_logical_chain(node: Node<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let mut first = None;
    let mut rest = Vec::new();
    flatten_logical_chain(node, ctx, &mut first, &mut rest);
    group(array(&[first, indent(array(&rest))]))
}

/// Flattens nested logical chain nodes (AndNode, OrNode) into a linear sequence of Documents.
fn flatten_logical_chain(
    node: Node<'_>,
    ctx: &mut BuildContext,
    first: &mut Option<Document>,
    rest: &mut Vec<Option<Document>>,
) {
    match node {
        Node::AndNode { .. } => {
            let and_node = node.as_and_node().unwrap();
            let left = and_node.left();
            let right = and_node.right();
            let operator = and_node.operator_loc();
            flatten_logical_chain(left, ctx, first, rest);
            rest.push(space());
            rest.push(build_location(operator, ctx));
            rest.push(line());
            rest.push(build_node(right, ctx));
        }
        Node::OrNode { .. } => {
            let or_node = node.as_or_node().unwrap();
            let left = or_node.left();
            let right = or_node.right();
            let operator = or_node.operator_loc();
            flatten_logical_chain(left, ctx, first, rest);
            rest.push(space());
            rest.push(build_location(operator, ctx));
            rest.push(line());
            rest.push(build_node(right, ctx));
        }
        _ => match first {
            None => {
                *first = build_node(node, ctx);
            }
            Some(_) => {
                unreachable!("Unexpected additional node in logical chain");
            }
        },
    }
}
