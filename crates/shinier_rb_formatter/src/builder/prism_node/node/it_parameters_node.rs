use crate::BuildContext;
use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::ItParametersNode;

pub fn build_node(node: Option<&ItParametersNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    node.as_node().build(context)
}
