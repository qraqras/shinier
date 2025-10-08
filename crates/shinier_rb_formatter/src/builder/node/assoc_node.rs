use crate::builder::build;
use crate::doc::{Doc, group, sequence, text};
use crate::space;
use ruby_prism::{AssocNode, Node};

const SYMBOL_SEPARATER: &str = ":";
const ROCKET_SEPARATER: &str = "=>";

pub fn build_node(node: &AssocNode) -> Doc {
    let key = node.key();
    let value = node.value();
    let separater_doc = match key {
        Node::SymbolNode { .. } => sequence(&[text(SYMBOL_SEPARATER), space()]),
        _ => sequence(&[space(), text(ROCKET_SEPARATER), space()]),
    };
    return group(&[build(&key), separater_doc, build(&value)]);
}
