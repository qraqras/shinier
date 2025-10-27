use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Location;

impl<'a> BuildPrismNode for Location<'_> {
    fn _build(&self, _context: &mut BuildContext) -> Document {
        match std::str::from_utf8(self.as_slice()) {
            Ok(s) => string(s),
            Err(_) => string("<invalid utf8>"),
        }
    }
}
