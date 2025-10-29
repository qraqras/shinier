use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::ItParametersNode;

impl<'sh> Build for Option<&ItParametersNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ItParametersNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    node.as_node().build(context)
}
