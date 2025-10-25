use crate::buildable::Buildable;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::helper::escape::escape;
use crate::keyword::DOUBLE_QUOTE;
use ruby_prism::InterpolatedStringNode;

pub fn build_node(node: Option<&InterpolatedStringNode>) -> Document {
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
                vec.push(part.build());
            }
        }
    }
    array(&[string(DOUBLE_QUOTE), array(&vec), string(DOUBLE_QUOTE)])
}
