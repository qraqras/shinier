use crate::BuildContext;
use crate::Build;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::SELF;
use ruby_prism::SelfNode;

impl<'sh> Build for Option<&SelfNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(_node: Option<&SelfNode>, _context: &mut BuildContext) -> Document {
    string(SELF)
}
