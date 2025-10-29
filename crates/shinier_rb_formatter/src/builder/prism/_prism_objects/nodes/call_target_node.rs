use crate::Build;
use crate::BuildContext;
use crate::builder::builder::array;
use crate::document::Document;
use crate::helper::build_receiver::build_receiver;
use ruby_prism::CallTargetNode;

impl<'sh> Build for Option<&CallTargetNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&CallTargetNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let is_safe_navigation = node.is_safe_navigation();
    let receiver = node.receiver();
    let name = node.name();
    array(&[
        build_receiver(Some(&receiver), is_safe_navigation, context),
        name.build(context),
    ])
}
