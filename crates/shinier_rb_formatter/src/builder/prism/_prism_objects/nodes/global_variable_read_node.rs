use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::GlobalVariableReadNode;

impl<'sh> Build for Option<&GlobalVariableReadNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&GlobalVariableReadNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    node.name().build(context)
}
