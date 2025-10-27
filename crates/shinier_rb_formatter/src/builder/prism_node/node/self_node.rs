use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::SELF;
use ruby_prism::SelfNode;

pub fn build_node(_node: Option<&SelfNode>, _context: &mut BuildContext) -> Document {
    string(SELF)
}
