use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::LocalVariableTargetNode;

impl<'sh> Build for LocalVariableTargetNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &LocalVariableTargetNode, context: &mut BuildContext) -> Document {
    let name = node.name();
    name.build(context)
}
