use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::helper::escape::escape;
use crate::keyword::BACK_QUOTE;
use ruby_prism::InterpolatedXStringNode;

pub fn build_node(node: Option<&InterpolatedXStringNode>, context: &mut BuildContext) -> Document {
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
    array(&[string(BACK_QUOTE), array(&vec), string(BACK_QUOTE)])
}
