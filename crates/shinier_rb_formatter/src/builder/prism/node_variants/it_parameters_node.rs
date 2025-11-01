use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::ItParametersNode;

impl<'sh> Build for ItParametersNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &ItParametersNode, context: &mut BuildContext) -> Document {
    node.as_node().build(context)
}
