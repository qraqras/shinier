use crate::Build;
use crate::BuildContext;
use crate::builder::builder::array;
use crate::builder::builder::string;
use crate::builder::prism::helper::escape::escape; // TODO: escape
use crate::document::Document;
use crate::keyword::COLON;
use ruby_prism::SymbolNode;

impl<'sh> Build for SymbolNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let unescaped = self.unescaped();
        match context.hash_label_style {
            true => string(escape(unescaped)),
            false => array(&[string(COLON), string(escape(unescaped))]),
        }
    }
}
