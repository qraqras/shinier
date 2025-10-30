use crate::Build;
use crate::BuildContext;
use crate::builder::builder::array;
use crate::builder::prism::helper::layout::build_logical_write;
use crate::builder::prism::helper::layout::build_receiver;
use crate::document::Document;
use crate::keyword::LogicalOperator;
use ruby_prism::CallOrWriteNode;

impl<'sh> Build for CallOrWriteNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &CallOrWriteNode, context: &mut BuildContext) -> Document {
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let read_name = node.read_name();
    let value = node.value();
    build_logical_write(
        array(&[
            build_receiver(receiver.as_ref(), is_safe_navigation, context),
            read_name.build(context),
        ]),
        value.build(context),
        LogicalOperator::Or,
    )
}
