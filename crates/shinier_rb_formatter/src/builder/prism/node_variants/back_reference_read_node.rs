use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::BackReferenceReadNode;

impl<'sh> Build for BackReferenceReadNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &BackReferenceReadNode, context: &mut BuildContext) -> Document {
    let name = node.name();
    name.build(context)
}
