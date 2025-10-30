use crate::Build;
use crate::BuildContext;
use crate::builder::builder::none;
use crate::document::Document;
use ruby_prism::ImplicitRestNode;

impl<'sh> Build for ImplicitRestNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(_node: &ImplicitRestNode, _context: &mut BuildContext) -> Document {
    none()
}
