use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::AlternationPatternNode;
use ruby_prism::Node;

pub fn build_alternation_pattern_node(node: &AlternationPatternNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let mut parts = Vec::new();
    flatten(node.as_node(), ctx, &mut parts);
    assert!(!parts.is_empty());
    let first = parts.remove(0);
    group(array(&[first, indent(array(&parts))]))
}

/// Flattens nested AlternationPatternNode into a linear sequence of Documents.
fn flatten(node: Node<'_>, ctx: &mut BuildContext, acc: &mut Vec<Option<Document>>) {
    match node {
        Node::AlternationPatternNode { .. } => {
            let alternation_pattern_node = node.as_alternation_pattern_node().unwrap();
            let left = alternation_pattern_node.left();
            let right = alternation_pattern_node.right();
            let operator = alternation_pattern_node.operator_loc();
            flatten(left, ctx, acc);
            acc.push(space());
            acc.push(build_location(operator, ctx));
            acc.push(line());
            acc.push(build_node(right, ctx));
        }
        _ => {
            acc.push(build_node(node, ctx));
        }
    }
}
