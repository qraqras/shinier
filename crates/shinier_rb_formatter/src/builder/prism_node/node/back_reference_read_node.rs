use crate::buildable::Buildable;
use crate::document::Document;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::BackReferenceReadNode;
use ruby_prism::Comments;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&BackReferenceReadNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build(comments)
}
