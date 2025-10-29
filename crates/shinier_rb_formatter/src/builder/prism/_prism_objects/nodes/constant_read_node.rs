use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::ConstantReadNode;

impl<'sh> Build for Option<&ConstantReadNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ConstantReadNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    node.name().build(context)
}
