use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::*;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&ItParametersNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    node.as_node().build(comments)
}
