use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::InstanceVariableTargetNode;

impl<'sh> Build for InstanceVariableTargetNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &InstanceVariableTargetNode, context: &mut BuildContext) -> Document {
    let name = node.name();
    name.build(context)
}
