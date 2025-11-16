use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::{Document, array, hardline};
use ruby_prism::StatementsNode;

pub fn build_statements_node(node: &StatementsNode<'_>, context: &mut BuildContext) -> Document {
    let mut parts = Vec::new();
    for (i, node) in node.body().iter().enumerate() {
        if i > 0 {
            parts.push(hardline());
        }
        parts.push(build_node(&node, context));
    }
    array(&parts)
}
