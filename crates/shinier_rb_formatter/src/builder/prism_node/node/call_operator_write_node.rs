use crate::builder::builder::array;
use crate::document::Document;
use crate::helper::build_receiver::build_receiver;
use crate::helper::build_write::build_operator_write;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::CallOperatorWriteNode;
use ruby_prism::Comments;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&CallOperatorWriteNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let read_name = node.read_name();
    let binary_operator = node.binary_operator();
    let value = node.value();
    build_operator_write(
        array(&[
            build_receiver(receiver.as_ref(), is_safe_navigation, comments),
            read_name.build(comments),
        ]),
        value.build(comments),
        binary_operator.build(comments),
    )
}
