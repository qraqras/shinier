use crate::Build;
use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::FILE;
use ruby_prism::SourceFileNode;

impl<'sh> Build for SourceFileNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(_node: &SourceFileNode, _context: &mut BuildContext) -> Document {
    string(FILE)
}
