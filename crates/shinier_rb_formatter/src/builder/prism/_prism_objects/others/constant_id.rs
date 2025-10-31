use crate::Build;
use crate::builder::BuildContext;
use crate::builder::builder::{none, string};
use crate::document::Document;
use ruby_prism::ConstantId;

impl<'a> Build for ConstantId<'_> {
    fn __build__(&self, _context: &mut BuildContext) -> Document {
        string(String::from_utf8(self.as_slice().to_vec()).unwrap())
    }
}
