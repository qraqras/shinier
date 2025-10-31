use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::LocalVariableReadNode;

impl<'sh> Build for LocalVariableReadNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &LocalVariableReadNode, context: &mut BuildContext) -> Document {
    let name = node.name();
    name.build(context)
}
