use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::ConstantListIter;

impl<'a> Buildable<'_> for ConstantListIter<'_> {
    fn build(&self) -> Document {
        unimplemented!("ConstantListIter");
    }
}
