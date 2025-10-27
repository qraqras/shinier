use crate::BuildPrismNode;
use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comment;
use ruby_prism::Comments;
use std::collections::HashMap;
use std::io::Read;
use std::iter::Peekable;

impl<'a> BuildPrismNode for Comment<'_> {
    fn _build(
        &self,
        comments: &mut Peekable<Comments>,
        option: Option<&HashMap<&str, bool>>,
    ) -> Document {
        let mut buf = String::new();
        self.text().read_to_string(&mut buf);
        string(buf)
    }
}
