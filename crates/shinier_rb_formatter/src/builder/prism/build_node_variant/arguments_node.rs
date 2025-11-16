use crate::Document;
use crate::builder::builder::*;
use crate::builder::keyword::COMMA;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ArgumentsNode;

pub fn build_arguments_node(node: &ArgumentsNode<'_>, context: &mut BuildContext) -> Document {
    let mut arguments = Vec::new();
    for (i, node) in node.arguments().iter().enumerate() {
        if i > 0 {
            arguments.push(string(COMMA));
            arguments.push(line());
        }
        arguments.push(build_node(&node, context));
    }
    group(array(&arguments))
}
