use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::IntegerNode;

impl<'sh> Build for IntegerNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &IntegerNode, context: &mut BuildContext) -> Document {
    let location = node.location();
    location.build(context)
}
