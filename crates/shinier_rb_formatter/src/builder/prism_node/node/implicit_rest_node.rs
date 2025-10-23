use crate::builder::builder::none;
use crate::document::Document;
use ruby_prism::ImplicitRestNode;

pub fn build_node(_node: Option<&ImplicitRestNode>) -> Document {
    none()
}
