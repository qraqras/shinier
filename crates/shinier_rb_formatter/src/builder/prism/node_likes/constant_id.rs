use crate::Build;
use crate::BuildContext;
use crate::Document;
use crate::builder::builder::string;
use ruby_prism::ConstantId;

impl<'sh> Build for ConstantId<'sh> {
    fn __build__(&self, _context: &mut BuildContext) -> Document {
        string(std::str::from_utf8(self.as_slice()).unwrap())
    }
}
