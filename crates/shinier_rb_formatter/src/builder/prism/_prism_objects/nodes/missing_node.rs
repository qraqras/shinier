use crate::BuildContext;
use crate::Build;
use crate::builder::builder::none;
use crate::document::Document;
use ruby_prism::MissingNode;

impl<'sh> Build for Option<&MissingNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(_node: Option<&MissingNode>, _context: &mut BuildContext) -> Document {
    none()
}
