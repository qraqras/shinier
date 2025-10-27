use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::none;
use crate::document::Document;
use ruby_prism::IntegerNode;

pub fn build_node(node: Option<&IntegerNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
            let location = node.location();
            location.build(context)
        }
        None => none(),
    }
}
