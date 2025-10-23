use crate::buildable::BuildableList;
use crate::builder::builder::{group, none, string};
use crate::document::Document;
use crate::keyword::SLASH;
use ruby_prism::InterpolatedRegularExpressionNode;

pub fn build_node(node: Option<&InterpolatedRegularExpressionNode>) -> Document {
    let node = node.unwrap();
    let parts = node.parts();
    group(parts.build_with(none(), Some(string(SLASH)), Some(string(SLASH))))
}
