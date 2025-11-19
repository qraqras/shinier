use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::helper::comment_helper::separate_indented_comments;
use crate::{Document, array, hardline};
use ruby_prism::StatementsNode;

pub fn build_statements_node(node: &StatementsNode<'_>, context: &mut BuildContext) -> Document {
    let mut dangling = context.dangling_comments.take();
    // Separate comments by their position relative to the statements block
    let (inner_dangling, outer_remaining) =
        separate_indented_comments(&node.as_node(), &mut dangling, &context.line_break_index).unwrap_or((None, None));

    let mut parts = Vec::new();
    for (i, stmt) in node.body().iter().enumerate() {
        if i > 0 {
            parts.push(hardline());
        }
        parts.push(build_node(&stmt, context));
    }

    // Comments after the statements belong to the inner block
    context.dangling_comments = inner_dangling;
    // Comments before the statements belong to the outer scope
    context.remaining_comments = outer_remaining;

    array(&parts)
}
