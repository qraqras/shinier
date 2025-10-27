use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::FloatNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&FloatNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let location = node.location();
    location.build(comments)
}
