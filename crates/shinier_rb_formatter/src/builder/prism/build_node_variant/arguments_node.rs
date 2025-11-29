use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ArgumentsNode;

/// Builds ArgumentsNode.
///
/// Arguments are separated by commas and line breaks.
pub fn build_arguments_node(node: &ArgumentsNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let arguments = node.arguments();
    let mut arguments_doc = Vec::new();
    for (i, node) in arguments.iter().enumerate() {
        if i > 0 {
            arguments_doc.push(comma());
            arguments_doc.push(line());
        }
        arguments_doc.push(build_node(node, ctx));
    }
    match arguments_doc.is_empty() {
        true => None,
        false => group(array(&arguments_doc)),
    }
}
