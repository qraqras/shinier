use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Doc;
use ruby_prism::ConstantId;

impl<'a> Buildable<'_> for ConstantId<'_> {
    fn build(&self) -> Doc {
        string(String::from_utf8(self.as_slice().to_vec()).unwrap())
    }
}
