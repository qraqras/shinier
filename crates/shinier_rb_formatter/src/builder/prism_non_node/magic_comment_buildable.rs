use crate::builder::Buildable;
use crate::document::Doc;
use ruby_prism::MagicComment;

impl<'a> Buildable<'_> for MagicComment<'_> {
    fn build(&self) -> Doc {
        unimplemented!("MagicComment");
    }
}
