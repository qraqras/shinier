use crate::builder::builder::none;
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::ImplicitRestNode;
use std::collections::HashMap;

pub fn build_node(_node: Option<&ImplicitRestNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    none()
}
