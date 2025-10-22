use crate::builder::Buildable;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::helper::build_symbol_without_colon::build_symbol_without_colon;
use crate::keyword::{COLON, ROCKET};
use ruby_prism::AssocNode;

pub fn build_node(node: Option<&AssocNode>) -> Document {
    let node = node.unwrap();
    let key = node.key();
    let value = node.value();
    match key.as_symbol_node() {
        Some(symbol_node) => group(array(&[
            build_symbol_without_colon(&symbol_node.as_node()),
            string(COLON),
            space(),
            value.build(),
        ])),
        None => group(array(&[
            key.build(),
            array(&[space(), string(ROCKET), space()]),
            value.build(),
        ])),
    }
}
