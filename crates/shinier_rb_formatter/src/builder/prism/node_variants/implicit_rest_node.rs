use crate::Build;
use crate::BuildContext;
use crate::builder::builder::string;
use crate::builder::keyword::COMMA;
use crate::document::Document;
use ruby_prism::ImplicitRestNode;

impl<'sh> Build for ImplicitRestNode<'sh> {
    fn __build__(&self, _context: &mut BuildContext) -> Document {
        string(COMMA)
    }
}
