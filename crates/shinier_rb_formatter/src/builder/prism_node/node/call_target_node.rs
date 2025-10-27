use crate::buildable::Buildable;
use crate::builder::builder::array;
use crate::document::Document;
use crate::helper::build_receiver::build_receiver;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::CallTargetNode;
use ruby_prism::Comments;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&CallTargetNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let name = node.name();
    array(&[
        build_receiver(Some(&receiver), is_safe_navigation, comments),
        name.build(comments),
    ])
}
