use crate::Build;
use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::FALSE;
use ruby_prism::FalseNode;

impl<'sh> Build for FalseNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(_node: &FalseNode, _context: &mut BuildContext) -> Document {
    string(FALSE)
}
