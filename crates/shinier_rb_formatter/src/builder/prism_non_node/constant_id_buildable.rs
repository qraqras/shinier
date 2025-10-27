use crate::BuildPrismNode;
use crate::builder::BuildContext;
use crate::builder::builder::{none, string};
use crate::document::Document;
use ruby_prism::ConstantId;

impl<'a> BuildPrismNode for ConstantId<'_> {
    fn _build(&self, _context: &mut BuildContext) -> Document {
        string(String::from_utf8(self.as_slice().to_vec()).unwrap())
    }
}

impl<'a> BuildPrismNode for Option<ConstantId<'_>> {
    fn _build(&self, _context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node._build(_context),
            None => none(),
        }
    }
}
