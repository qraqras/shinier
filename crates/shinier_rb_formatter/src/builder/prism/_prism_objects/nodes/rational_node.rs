use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::RationalNode;

impl<'sh> Build for RationalNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &RationalNode, context: &mut BuildContext) -> Document {
    let location = node.location();
    location.build(context)
}
