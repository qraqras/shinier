use crate::builder::Buildable;
use crate::doc::{Doc, none, text};
use ruby_prism::ConstantId;

impl<'a> Buildable<'_> for ConstantId<'_> {
    fn build(&self) -> Doc {
        text(String::from_utf8(self.as_slice().to_vec()).unwrap())
    }
}
impl<'a> Buildable<'_> for Option<ConstantId<'a>> {
    fn build(&self) -> Doc {
        match self {
            Some(s) => s.build(),
            None => none(),
        }
    }
}
