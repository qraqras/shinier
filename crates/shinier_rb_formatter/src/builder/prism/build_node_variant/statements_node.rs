use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::helper::comment_helper::update_dangling_remaining;
use crate::{Document, array, hardline};
use ruby_prism::StatementsNode;

pub fn build_statements_node(node: &StatementsNode<'_>, context: &mut BuildContext) -> Document {
    // let mut dangling = context.dangling_comments.take();
    // let mut remaining = context.remaining_comments.take();

    let mut parts = Vec::new();
    for (i, stmt) in node.body().iter().enumerate() {
        if i > 0 {
            parts.push(hardline());
        }
        parts.push(build_node(&stmt, context));
    }

    // update_dangling_remaining(&mut dangling, &mut remaining, &node.as_node(), context);
    // context.dangling_comments = dangling;
    // context.remaining_comments = remaining;

    array(&parts)
}
