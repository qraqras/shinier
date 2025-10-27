use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::LINE;
use ruby_prism::SourceLineNode;

pub fn build_node(_node: Option<&SourceLineNode>, _context: &mut BuildContext) -> Document {
    string(LINE)
}
