use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::MagicComments;

impl<'a> Buildable<'_> for MagicComments<'_> {
    fn build(&self) -> Document {
        unimplemented!("MagicComments");
    }
}
