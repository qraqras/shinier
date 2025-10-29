use crate::BuildContext;
use crate::Build;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::RETRY;
use ruby_prism::RetryNode;

impl<'sh> Build for Option<&RetryNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(_node: Option<&RetryNode>, _context: &mut BuildContext) -> Document {
    string(RETRY)
}
