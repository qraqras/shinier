use crate::BuildContext;
use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::FloatNode;

pub fn build_node(node: Option<&FloatNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let location = node.location();
    location.build(context)
}
