use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use crate::helper::build_receiver::build_receiver;
use crate::helper::build_write::build_operator_write;
use ruby_prism::CallOperatorWriteNode;

pub fn build_node(node: Option<&CallOperatorWriteNode>) -> Document {
    let node = node.unwrap();
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let read_name = node.read_name();
    let binary_operator = node.binary_operator();
    let value = node.value();
    build_operator_write(
        array(&[
            build_receiver(receiver.as_ref(), is_safe_navigation),
            read_name.build(),
        ]),
        value.build(),
        binary_operator.build(),
    )
}
