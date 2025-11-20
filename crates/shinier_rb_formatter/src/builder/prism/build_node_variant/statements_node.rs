use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::helper::comment_helper::update_dangling_remaining;
use crate::{Document, array, hardline};
use ruby_prism::StatementsNode;

pub fn build_statements_node(node: &StatementsNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    // let mut dangling = ctx.dangling_comments.take();
    // let mut remaining = ctx.remaining_comments.take();

    let mut parts = Vec::new();
    for (i, stmt) in node.body().iter().enumerate() {
        if i > 0 {
            parts.push(hardline());
        }
        parts.push(build_node(&stmt, ctx));
    }

    // update_dangling_remaining(&mut dangling, &mut remaining, &node.as_node(), ctx);
    // ctx.dangling_comments = dangling;
    // ctx.remaining_comments = remaining;

    array(&parts)
}
