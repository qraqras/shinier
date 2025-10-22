use crate::builder::Buildable;
use crate::builder::builder::array;
use crate::document::Document;
use crate::helper::build_receiver::build_receiver;
use ruby_prism::CallTargetNode;

pub fn build_node(node: Option<&CallTargetNode>) -> Document {
    let node = node.unwrap();
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let name = node.name();
    array(&[
        build_receiver(Some(&receiver), is_safe_navigation),
        name.build(),
    ])
}
