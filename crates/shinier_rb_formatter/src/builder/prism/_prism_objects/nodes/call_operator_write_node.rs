use crate::Build;
use crate::BuildContext;
use crate::builder::builder::array;
use crate::document::Document;
use crate::helper::build_receiver::build_receiver;
use crate::helper::build_write::build_operator_write;
use ruby_prism::CallOperatorWriteNode;

impl<'sh> Build for CallOperatorWriteNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &CallOperatorWriteNode, context: &mut BuildContext) -> Document {
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let read_name = node.read_name();
    let binary_operator = node.binary_operator();
    let value = node.value();
    build_operator_write(
        array(&[
            build_receiver(receiver.as_ref(), is_safe_navigation, context),
            read_name.build(context),
        ]),
        value.build(context),
        binary_operator.build(context),
    )
}
