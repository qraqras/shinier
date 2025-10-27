use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::helper::escape::escape;
use crate::keyword::{COLON, DOUBLE_QUOTE};
use ruby_prism::InterpolatedSymbolNode;

pub fn build_node(node: Option<&InterpolatedSymbolNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let parts = node.parts();

    let mut vec = Vec::new();
    for part in parts.iter() {
        match part.as_string_node() {
            Some(string_node) => {
                let unescaped = string_node.unescaped();
                vec.push(string(escape(unescaped)));
            }
            None => {
                vec.push(part.build(context));
            }
        }
    }
    array(&[
        string(COLON),
        string(DOUBLE_QUOTE),
        array(&vec),
        string(DOUBLE_QUOTE),
    ])
}
