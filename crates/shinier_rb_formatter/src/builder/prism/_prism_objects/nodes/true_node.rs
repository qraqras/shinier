use crate::Build;
use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::TRUE;
use ruby_prism::TrueNode;

impl<'sh> Build for TrueNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(_node: &TrueNode, _context: &mut BuildContext) -> Document {
    string(TRUE)
}
