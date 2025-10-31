/*
use crate::Build;
use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comment;
use std::io::Read;

impl<'sh> Build for Comment<'sh> {
    fn __build__(&self, _context: &mut crate::BuildContext) -> Document {
        let mut buf = String::new();
        self.text().read_to_string(&mut buf);
        string(buf)
    }
}
*/
