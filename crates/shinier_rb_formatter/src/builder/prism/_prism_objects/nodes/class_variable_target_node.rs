use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::ClassVariableTargetNode;

impl<'sh> Build for Option<&ClassVariableTargetNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ClassVariableTargetNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build(context)
}
