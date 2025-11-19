use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::{Document, array, hardline};
use ruby_prism::StatementsNode;

pub fn build_statements_node(node: &StatementsNode<'_>, context: &mut BuildContext) -> Document {
    // TODO: Handle comments between statements
    let dangling = context.dangling_comments.take();

    let mut parts = Vec::new();
    for (i, node) in node.body().iter().enumerate() {
        if i > 0 {
            parts.push(hardline());
        }
        parts.push(build_node(&node, context));
    }

    // TODO: Handle dangling comments properly
    context.remaining_comments = dangling;

    array(&parts)
}
