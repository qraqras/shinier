use crate::Document;
use crate::builder::builder::*;
use crate::builder::keyword::RETURN;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ReturnNode;

pub fn build_return_node(node: &ReturnNode<'_>, context: &mut BuildContext) -> Document {
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    group(array(&[string(RETURN), space(), indent_opt(arguments)]))
}
