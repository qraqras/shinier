use crate::Document;
use crate::builder::builder::*;
use crate::builder::keyword::RETURN;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ReturnNode;

pub fn build_return_node(node: &ReturnNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let arguments = match &node.arguments() {
        Some(node) => build_node(&node.as_node(), ctx),
        None => None,
    };
    group(array(&[string(RETURN), space(), indent(arguments)]))
}
