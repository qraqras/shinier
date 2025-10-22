use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use ruby_prism::ConstantId;

impl<'a> Buildable<'_> for ConstantId<'_> {
    fn build(&self) -> Document {
        string(String::from_utf8(self.as_slice().to_vec()).unwrap())
    }
}
