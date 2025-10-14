use crate::builder::Buildable;
use crate::doc::{Doc, sequence};
use crate::helper::receiver::build_receiver;
use crate::helper::write_node::build_logical_write_node;
use crate::keyword::LogicalOperator;
use ruby_prism::CallOrWriteNode;

pub fn build_node(node: Option<&CallOrWriteNode>) -> Doc {
    let node = node.unwrap();
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let read_name = node.read_name();
    let value = node.value();
    build_logical_write_node(
        sequence(&[
            build_receiver(receiver.as_ref(), is_safe_navigation),
            read_name.build(),
        ]),
        value.build(),
        LogicalOperator::Or,
    )
}
