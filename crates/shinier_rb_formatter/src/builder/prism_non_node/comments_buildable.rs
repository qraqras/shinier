use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::Comments;

impl<'a> Buildable<'_> for Comments<'_> {
    fn build(&self) -> Document {
        unimplemented!("Comments");
    }
}
