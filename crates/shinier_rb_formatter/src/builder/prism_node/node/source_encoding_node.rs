use crate::BuildContext;
use crate::ENCODING;
use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::SourceEncodingNode;

pub fn build_node(_node: Option<&SourceEncodingNode>, _context: &mut BuildContext) -> Document {
    string(ENCODING)
}
