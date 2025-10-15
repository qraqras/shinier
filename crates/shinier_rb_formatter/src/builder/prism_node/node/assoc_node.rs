use crate::builder::Buildable;
use crate::doc::{Doc, group, sequence, space, text};
use crate::helper::build_symbol_without_colon::build_symbol_without_colon;
use crate::keyword::{COLON, ROCKET};
use ruby_prism::AssocNode;

pub fn build_node(node: Option<&AssocNode>) -> Doc {
    let node = node.unwrap();
    let key = node.key();
    let value = node.value();
    match key.as_symbol_node() {
        Some(symbol_node) => group(&[
            build_symbol_without_colon(&symbol_node.as_node()),
            text(COLON),
            space(),
            value.build(),
        ]),
        None => group(&[
            key.build(),
            sequence(&[space(), text(ROCKET), space()]),
            value.build(),
        ]),
    }
}
