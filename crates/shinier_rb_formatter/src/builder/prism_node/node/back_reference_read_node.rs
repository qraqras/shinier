use crate::document::Document;
use crate::{BuildContext, BuildPrismNode};
use ruby_prism::BackReferenceReadNode;

pub fn build_node(node: Option<&BackReferenceReadNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let name = node.name();
    name.build(context)
}
