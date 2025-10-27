use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::REDO;
use ruby_prism::RedoNode;

pub fn build_node(_node: Option<&RedoNode>, _context: &mut BuildContext) -> Document {
    string(REDO)
}
