use crate::Build;
use crate::BuildContext;
use crate::builder::builder::none;
use crate::document::Document;
use ruby_prism::IntegerNode;

impl<'sh> Build for Option<&IntegerNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&IntegerNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
            let location = node.location();
            location.build(context)
        }
        None => none(),
    }
}
