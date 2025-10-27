use crate::BuildContext;
use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::ImplicitNode;

pub fn build_node(node: Option<&ImplicitNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let value = node.value();
    value.build(context)
}
