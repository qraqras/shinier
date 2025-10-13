use crate::builder::Buildable;
use crate::doc::Doc;
use ruby_prism::ConstantListIter;

impl<'a> Buildable<'_> for ConstantListIter<'_> {
    fn build(&self) -> Doc {
        unimplemented!("ConstantListIter");
    }
}
