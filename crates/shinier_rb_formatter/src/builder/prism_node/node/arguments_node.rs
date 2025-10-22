use crate::BuildableList;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::COMMA;
use ruby_prism::ArgumentsNode;

pub fn build_node(node: Option<&ArgumentsNode>) -> Doc {
    match node {
        Some(node) => {
            let arguments = node.arguments();
            let separator = array(&[string(COMMA), line()]);
            arguments.build(separator, array)
        }
        None => none(),
    }
}
