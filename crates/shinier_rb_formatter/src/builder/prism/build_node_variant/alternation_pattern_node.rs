use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::AlternationPatternNode;
use ruby_prism::Node;

/// Builds AlternationPatternNode.
///
/// Flattens nested alternation patterns into a linear sequence of documents,
/// formatting them with proper indentation and line breaks.
pub fn build_alternation_pattern_node(node: &AlternationPatternNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let mut first = None;
    let mut rest = Vec::new();
    flatten(node.as_node(), ctx, &mut first, &mut rest);
    group(array(&[first, indent(array(&rest))]))
}

/// Flattens nested AlternationPatternNode into a linear sequence of Documents.
fn flatten(node: Node<'_>, ctx: &mut BuildContext, first: &mut Option<Document>, rest: &mut Vec<Option<Document>>) {
    match node {
        Node::AlternationPatternNode { .. } => {
            let alternation_pattern_node = node.as_alternation_pattern_node().unwrap();
            let left = alternation_pattern_node.left();
            let right = alternation_pattern_node.right();
            let operator = alternation_pattern_node.operator_loc();
            flatten(left, ctx, first, rest);
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
                unreachable!("Unexpected additional node in alternation pattern");
            }
        },
    }
}
