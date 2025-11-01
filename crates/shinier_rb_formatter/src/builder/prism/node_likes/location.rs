use crate::Build;
use crate::BuildContext;
use crate::Document;
use crate::builder::builder::string;
use ruby_prism::Location;

impl<'sh> Build for Location<'sh> {
    fn __build__(&self, _context: &mut BuildContext) -> Document {
        string(std::str::from_utf8(self.as_slice()).unwrap())
    }
}
