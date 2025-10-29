use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::RationalNode;

impl<'sh> Build for Option<&RationalNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&RationalNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let location = node.location();
    location.build(context)
}
