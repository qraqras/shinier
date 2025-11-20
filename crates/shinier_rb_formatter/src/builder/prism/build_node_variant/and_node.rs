use crate::Document;
use crate::builder::builder::*;
use crate::builder::keyword::LOGICAL_AND;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_custom_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::AndNode;
use ruby_prism::Node;

pub fn build_and_node(node: &AndNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let mut parts = Vec::new();
    flatten(&node.as_node(), ctx, &mut parts);
    assert!(!parts.is_empty());
    let first = parts.remove(0);
    group(array(&[first, indent(array(&parts))]))
}

fn flatten(node: &Node<'_>, ctx: &mut BuildContext, acc: &mut Vec<Option<Document>>) {
    match node {
        Node::AndNode { .. } => {
            let and_node = node.as_and_node().unwrap();
            let left = and_node.left();
            let right = and_node.right();
            let operator = and_node.operator_loc();
            flatten(&left, ctx, acc);
            acc.push(space());
            acc.push(build_custom_location(&operator, ctx, LOGICAL_AND));
            acc.push(line());
            acc.push(build_node(&right, ctx));
        }
        _ => {
            acc.push(build_node(node, ctx));
        }
    }
}
