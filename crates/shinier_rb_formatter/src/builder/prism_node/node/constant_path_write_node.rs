use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use crate::helper::build_write::build_write;
use ruby_prism::ConstantPathWriteNode;

impl<'sh> Build for Option<&ConstantPathWriteNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ConstantPathWriteNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    build_write(target.as_node().build(context), value.build(context))
}
