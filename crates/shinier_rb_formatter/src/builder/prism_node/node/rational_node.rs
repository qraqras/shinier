use crate::BuildContext;
use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::RationalNode;

pub fn build_node(node: Option<&RationalNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let location = node.location();
    location.build(context)
}
