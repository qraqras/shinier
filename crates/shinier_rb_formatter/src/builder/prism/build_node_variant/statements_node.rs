use crate::Document;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::statements_node::LayoutParamStatementsNode;
use crate::builder::prism::layout_node_variant::statements_node::layout_statements_node;
use ruby_prism::StatementsNode;

pub fn build_statements_node(node: &StatementsNode<'_>, context: &mut BuildContext) -> Document {
    let mut body = Vec::new();
    for node in &node.body() {
        body.push(build_node(&node, context));
    }
    layout_statements_node(LayoutParamStatementsNode { body })
}
