use crate::builder::Buildable;
use crate::doc::Doc;
use ruby_prism::MagicComment;

impl<'a> Buildable<'_> for MagicComment<'_> {
    fn build(&self) -> Doc {
        unimplemented!("MagicComment");
    }
}
