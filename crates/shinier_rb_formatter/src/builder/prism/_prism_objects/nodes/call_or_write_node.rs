use crate::Build;
use crate::BuildContext;
use crate::builder::builder::array;
use crate::document::Document;
use crate::helper::build_receiver::build_receiver;
use crate::helper::build_write::build_logical_write;
use crate::keyword::LogicalOperator;
use ruby_prism::CallOrWriteNode;

impl<'sh> Build for Option<&CallOrWriteNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&CallOrWriteNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
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
