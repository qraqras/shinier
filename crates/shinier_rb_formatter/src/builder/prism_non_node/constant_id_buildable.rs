use crate::builder::Buildable;
use crate::doc::{Doc, text};
use ruby_prism::ConstantId;

impl<'a> Buildable<'_> for ConstantId<'_> {
    fn build(&self) -> Doc {
        text(String::from_utf8(self.as_slice().to_vec()).unwrap())
    }
}
