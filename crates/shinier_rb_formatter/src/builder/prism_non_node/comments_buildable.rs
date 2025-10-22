use crate::builder::Buildable;
use crate::document::Doc;
use ruby_prism::Comments;

impl<'a> Buildable<'_> for Comments<'_> {
    fn build(&self) -> Doc {
        unimplemented!("Comments");
    }
}
