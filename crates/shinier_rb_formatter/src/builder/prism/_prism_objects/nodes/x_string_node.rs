use crate::BuildContext;
use crate::Build;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::helper::escape::escape;
use crate::keyword::BACK_QUOTE;
use ruby_prism::XStringNode;

impl<'sh> Build for Option<&XStringNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&XStringNode>, _context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let unescaped = node.unescaped();
    array(&[
        string(BACK_QUOTE),
        string(escape(unescaped)),
        string(BACK_QUOTE),
    ])
}
