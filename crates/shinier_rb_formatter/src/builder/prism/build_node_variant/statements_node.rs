use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::{Document, array, hardline};
use ruby_prism::StatementsNode;

pub fn build_statements_node(node: &StatementsNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let mut parts = Vec::new();
    for (i, stmt) in node.body().iter().enumerate() {
        if i > 0 {
            parts.push(hardline());
        }
        parts.push(build_node(stmt, ctx));
    }
    array(&parts)
}
