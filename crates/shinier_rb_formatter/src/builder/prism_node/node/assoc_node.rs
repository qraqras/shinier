use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::helper::build_symbol_without_colon::build_symbol_without_colon;
use crate::keyword::{COLON, ROCKET};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::AssocNode;
use ruby_prism::Comments;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&AssocNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let key = node.key();
    let value = node.value();
    match key.as_symbol_node() {
        Some(symbol_node) => group(array(&[
            build_symbol_without_colon(&symbol_node.as_node()),
            string(COLON),
            space(),
            value.build(comments),
        ])),
        None => group(array(&[
            key.build(comments),
            array(&[space(), string(ROCKET), space()]),
            value.build(comments),
        ])),
    }
}
