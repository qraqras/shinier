use crate::builder::Buildable;
use crate::doc::Doc;
use ruby_prism::MagicComments;

impl<'a> Buildable<'_> for MagicComments<'_> {
    fn build(&self) -> Doc {
        unimplemented!("MagicComments");
    }
}
