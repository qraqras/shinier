use crate::BuildableList;
use crate::doc::{Doc, line, none, sequence, text};
use crate::keyword::COMMA;
use ruby_prism::ArgumentsNode;

pub fn build_node(node: Option<&ArgumentsNode>) -> Doc {
    match node {
        Some(node) => {
            let arguments = node.arguments();
            arguments.build_group(&sequence(&[text(COMMA), line()]))
        }
        None => none(),
    }
}
