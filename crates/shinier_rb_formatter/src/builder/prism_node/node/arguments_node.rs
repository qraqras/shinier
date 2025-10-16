use crate::BuildableList;
use crate::doc::{Doc, group, line, none, sequence, text};
use crate::keyword::COMMA;
use ruby_prism::ArgumentsNode;

pub fn build_node(node: Option<&ArgumentsNode>) -> Doc {
    match node {
        Some(node) => {
            let arguments = node.arguments();
            let separator = sequence(&[text(COMMA), line()]);
            arguments.build(separator, group)
        }
        None => none(),
    }
}
