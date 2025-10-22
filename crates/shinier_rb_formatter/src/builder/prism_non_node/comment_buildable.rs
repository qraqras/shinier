use crate::builder::Buildable;
use crate::document::Doc;
use ruby_prism::Comment;

impl<'a> Buildable<'_> for Comment<'_> {
    fn build(&self) -> Doc {
        unimplemented!("Comment");
    }
}
