use crate::BuildContext;
use crate::Build;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::helper::escape::escape;
use crate::keyword::COLON;
use ruby_prism::SymbolNode;

impl<'sh> Build for Option<&SymbolNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&SymbolNode>, _context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let unescaped = node.unescaped();
    array(&[string(COLON), string(escape(unescaped))])
}
