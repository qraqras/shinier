use crate::BuildContext;
use crate::Build;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::REDO;
use ruby_prism::RedoNode;

impl<'sh> Build for Option<&RedoNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(_node: Option<&RedoNode>, _context: &mut BuildContext) -> Document {
    string(REDO)
}
