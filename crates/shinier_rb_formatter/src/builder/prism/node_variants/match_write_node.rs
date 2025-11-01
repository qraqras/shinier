use crate::Build;
use crate::BuildContext;
use crate::builder::builder::group;
use crate::document::Document;
use ruby_prism::MatchWriteNode;

impl<'sh> Build for MatchWriteNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &MatchWriteNode, context: &mut BuildContext) -> Document {
    let call = node.call();
    group(call.as_node().build(context))
}
