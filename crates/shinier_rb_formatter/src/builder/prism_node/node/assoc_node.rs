use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::helper::build_symbol_without_colon::build_symbol_without_colon;
use crate::keyword::{COLON, ROCKET};
use ruby_prism::AssocNode;

pub fn build_node(node: Option<&AssocNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let key = node.key();
    let value = node.value();
    match key.as_symbol_node() {
        Some(symbol_node) => group(array(&[
            build_symbol_without_colon(&symbol_node.as_node()),
            string(COLON),
            space(),
            value.build(context),
        ])),
        None => group(array(&[
            key.build(context),
            array(&[space(), string(ROCKET), space()]),
            value.build(context),
        ])),
    }
}
