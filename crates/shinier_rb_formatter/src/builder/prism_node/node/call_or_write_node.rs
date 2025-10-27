use crate::buildable::Buildable;
use crate::builder::builder::array;
use crate::document::Document;
use crate::helper::build_receiver::build_receiver;
use crate::helper::build_write::build_logical_write;
use crate::keyword::LogicalOperator;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::CallOrWriteNode;
use ruby_prism::Comments;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&CallOrWriteNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let read_name = node.read_name();
    let value = node.value();
    build_logical_write(
        array(&[
            build_receiver(receiver.as_ref(), is_safe_navigation, comments),
            read_name.build(comments),
        ]),
        value.build(comments),
        LogicalOperator::Or,
    )
}
