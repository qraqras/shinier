use crate::BuildPrismNode;
use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comment;
use std::io::Read;

impl<'a> BuildPrismNode for Comment<'_> {
    fn _build(&self, _context: &mut crate::BuildContext) -> Document {
        let mut buf = String::new();
        self.text().read_to_string(&mut buf);
        string(buf)
    }
}
