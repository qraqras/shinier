use crate::Build;
use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::TRIPLE_DOT;
use ruby_prism::ForwardingParameterNode;

impl<'sh> Build for ForwardingParameterNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(_node: &ForwardingParameterNode, _context: &mut BuildContext) -> Document {
    string(TRIPLE_DOT)
}
