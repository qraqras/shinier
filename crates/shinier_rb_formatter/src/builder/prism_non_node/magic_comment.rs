use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::MagicComment;

impl<'a> Buildable<'_> for MagicComment<'_> {
    fn build(&self) -> Document {
        unimplemented!("MagicComment");
    }
}
