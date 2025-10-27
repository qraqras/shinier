use crate::BuildPrismNode;
use crate::document::Document;

use ruby_prism::ConstantReadNode;

use crate::BuildContext;

pub fn build_node(node: Option<&ConstantReadNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    node.name().build(context)
}
