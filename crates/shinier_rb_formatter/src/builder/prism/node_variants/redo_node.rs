use crate::Build;
use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::REDO;
use ruby_prism::RedoNode;

impl<'sh> Build for RedoNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(_node: &RedoNode, _context: &mut BuildContext) -> Document {
    string(REDO)
}
