use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::helper::escape::escape;
use crate::keyword::DOUBLE_QUOTE;
use ruby_prism::InterpolatedStringNode;

impl<'sh> Build for Option<&InterpolatedStringNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&InterpolatedStringNode>, context: &mut BuildContext) -> Document {
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
    array(&[string(DOUBLE_QUOTE), array(&vec), string(DOUBLE_QUOTE)])
}
