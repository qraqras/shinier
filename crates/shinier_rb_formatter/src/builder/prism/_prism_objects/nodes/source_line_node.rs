use crate::Build;
use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::LINE;
use ruby_prism::SourceLineNode;

impl<'sh> Build for SourceLineNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(_node: &SourceLineNode, _context: &mut BuildContext) -> Document {
    string(LINE)
}
