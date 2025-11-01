use crate::Build;
use crate::BuildContext;
use crate::ENCODING;
use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::SourceEncodingNode;

impl<'sh> Build for SourceEncodingNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(_node: &SourceEncodingNode, _context: &mut BuildContext) -> Document {
    string(ENCODING)
}
