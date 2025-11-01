use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::GlobalVariableReadNode;

impl<'sh> Build for GlobalVariableReadNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &GlobalVariableReadNode, context: &mut BuildContext) -> Document {
    node.name().build(context)
}
