use crate::builder::builder::none;
use crate::document::Document;
use ruby_prism::MissingNode;

pub fn build_node(_node: Option<&MissingNode>) -> Document {
    none()
}
