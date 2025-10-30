use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::builder::prism::helper::escape::escape;
use crate::keyword::SLASH;
use ruby_prism::InterpolatedRegularExpressionNode;

impl<'sh> Build for InterpolatedRegularExpressionNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(
    node: &InterpolatedRegularExpressionNode,
    context: &mut BuildContext,
) -> Document {
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
    array(&[string(SLASH), array(&vec), string(SLASH)])
}
