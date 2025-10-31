use crate::Build;
use crate::BuildContext;
use crate::builder::builder::array;
use crate::builder::prism::helper::layout::build_receiver;
use crate::document::Document;
use ruby_prism::CallTargetNode;

impl<'sh> Build for CallTargetNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &CallTargetNode, context: &mut BuildContext) -> Document {
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let name = node.name();
    array(&[
        build_receiver(Some(&receiver), is_safe_navigation, context),
        name.build(context),
    ])
}
