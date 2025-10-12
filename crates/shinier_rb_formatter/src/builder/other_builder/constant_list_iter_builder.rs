use crate::builder::Buildable;
use crate::doc::{Doc, text};
use ruby_prism::ConstantListIter;

impl<'a> Buildable<'_> for ConstantListIter<'_> {
    fn build(&self) -> Doc {
        text(format!(
            "not implemented: {:?}",
            std::any::type_name_of_val(self)
        ))
    }
}
