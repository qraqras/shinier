use crate::ast_to_doc::printer;
use crate::doc::{Doc, sequence, text};
use ruby_prism::SplatNode;

pub fn print(node: &SplatNode) -> Doc {
    if let Some(node) = node.expression() {
        return sequence(vec![text("*"), printer::print(&node)]);
    }
    Doc::default()
}
