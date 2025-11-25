use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ArgumentsNode;

/// Builds ArgumentsNode.
///
/// Arguments are separated by commas and line breaks.
pub fn build_arguments_node(node: &ArgumentsNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let arguments = node.arguments();
    let mut documents = Vec::new();
    for (i, node) in arguments.iter().enumerate() {
        if i > 0 {
            documents.push(comma());
            documents.push(line());
        }
        documents.push(build_node(node, ctx));
    }
    group(array(&documents))
}
