use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::group;
use crate::document::Document;
use ruby_prism::MatchWriteNode;

pub fn build_node(node: Option<&MatchWriteNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let call = node.call();
    group(call.as_node().build(context))
}
