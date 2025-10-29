use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::helper::build_symbol_without_colon::build_symbol_without_colon;
use crate::keyword::{COLON, ROCKET};
use ruby_prism::AssocNode;

impl<'sh> Build for Option<&AssocNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

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
