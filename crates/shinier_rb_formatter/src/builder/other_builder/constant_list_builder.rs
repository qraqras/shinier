use crate::builder::Buildable;
use crate::doc::Doc;
use ruby_prism::ConstantList;

impl<'a> Buildable<'_> for ConstantList<'_> {
    fn build(&self) -> Doc {
        unimplemented!("ConstantList");
    }
}
