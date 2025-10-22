use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::Comment;

impl<'a> Buildable<'_> for Comment<'_> {
    fn build(&self) -> Document {
        unimplemented!("Comment");
    }
}
