use crate::Build;
use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::TRIPLE_DOT;
use ruby_prism::ForwardingArgumentsNode;

impl<'sh> Build for ForwardingArgumentsNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(_node: &ForwardingArgumentsNode, _context: &mut BuildContext) -> Document {
    string(TRIPLE_DOT)
}
