use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, string};
use crate::builder::prism::helper::escape::escape;
use crate::document::Document;
use crate::keyword::COLON;
use ruby_prism::SymbolNode;

impl<'sh> Build for SymbolNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &SymbolNode, _context: &mut BuildContext) -> Document {
    let unescaped = node.unescaped();
    array(&[string(COLON), string(escape(unescaped))])
}
