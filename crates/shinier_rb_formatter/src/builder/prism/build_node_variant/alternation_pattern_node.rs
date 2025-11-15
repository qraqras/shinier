use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::AlternationPatternNode;
use ruby_prism::Node;

pub fn build_alternation_pattern_node(
    node: &AlternationPatternNode<'_>,
    context: &mut BuildContext,
) -> Document {
    let mut acc = Vec::new();
    flatten(&node.as_node(), context, &mut acc);
    group(array(&acc))
}

fn flatten(node: &Node<'_>, context: &mut BuildContext, acc: &mut Vec<Document>) {
    match node {
        Node::AlternationPatternNode { .. } => {
            let alternation_pattern_node = node.as_alternation_pattern_node().unwrap();
            let left = alternation_pattern_node.left();
            let right = alternation_pattern_node.right();
            let operator = alternation_pattern_node.operator_loc();
            flatten(&left, context, acc);
            acc.push(space());
            acc.push(build_location(&operator, context));
            acc.push(indent(array(&[line(), build_node(&right, context)])));
        }
        _ => {
            acc.push(build_node(node, context));
        }
    }
}
