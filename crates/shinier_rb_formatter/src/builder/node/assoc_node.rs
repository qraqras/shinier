use crate::builder::build;
use crate::doc::{Doc, group, sequence, space, text};
use crate::keyword::{COLON, ROCKET};
use ruby_prism::{AssocNode, Node};

pub fn build_node(node: Option<&AssocNode>) -> Doc {
    let node = node.unwrap();
    let key = node.key();
    let value = node.value();
    let separator = match key {
        Node::SymbolNode { .. } => sequence(&[text(COLON), space()]),
        _ => sequence(&[space(), text(ROCKET), space()]),
    };
    group(&[build(&key), separator, build(&value)])
}
