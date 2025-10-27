use crate::BuildContext;
use crate::builder::builder::none;
use crate::document::Document;
use ruby_prism::MissingNode;

pub fn build_node(_node: Option<&MissingNode>, _context: &mut BuildContext) -> Document {
    none()
}
