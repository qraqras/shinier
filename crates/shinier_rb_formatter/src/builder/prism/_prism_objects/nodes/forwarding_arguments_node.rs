use crate::BuildContext;
use crate::Build;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::TRIPLE_DOT;
use ruby_prism::ForwardingArgumentsNode;

impl<'sh> Build for Option<&ForwardingArgumentsNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ForwardingArgumentsNode>, _context: &mut BuildContext) -> Document {
    let _node = node.unwrap();
    string(TRIPLE_DOT)
}
