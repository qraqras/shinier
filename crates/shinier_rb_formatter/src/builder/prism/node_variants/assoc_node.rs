use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::{COLON, ROCKET};
use ruby_prism::AssocNode;

impl<'sh> Build for AssocNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &AssocNode, context: &mut BuildContext) -> Document {
    let key = node.key();
    let value = node.value();
    match key.as_symbol_node() {
        Some(symbol_node) => group(array(&[
            symbol_node.build(context),
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
