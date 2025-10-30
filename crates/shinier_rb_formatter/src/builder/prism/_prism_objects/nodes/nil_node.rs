use crate::Build;
use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::NIL;
use ruby_prism::NilNode;

impl<'sh> Build for NilNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(_node: &NilNode, _context: &mut BuildContext) -> Document {
    string(NIL)
}
