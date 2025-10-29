use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use crate::helper::build_write::build_logical_write;
use crate::keyword::LogicalOperator;
use ruby_prism::ConstantPathOrWriteNode;

impl<'sh> Build for Option<&ConstantPathOrWriteNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ConstantPathOrWriteNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    build_logical_write(
        target.as_node().build(context),
        value.build(context),
        LogicalOperator::Or,
    )
}
