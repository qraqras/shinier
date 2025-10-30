use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::GlobalVariableTargetNode;

impl<'sh> Build for GlobalVariableTargetNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &GlobalVariableTargetNode, context: &mut BuildContext) -> Document {
    let name = node.name();
    name.build(context)
}
