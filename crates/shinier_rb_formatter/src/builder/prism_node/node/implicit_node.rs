use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::ImplicitNode;

impl<'sh> Build for Option<&ImplicitNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ImplicitNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let value = node.value();
    value.build(context)
}
