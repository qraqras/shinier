use crate::builder::Buildable;
use crate::doc::{Doc, none, text};
use ruby_prism::ConstantId;

impl<'a> Buildable<'_> for ConstantId<'_> {
    fn build(&self) -> Doc {
        text(format!(
            "not implemented: {:?}",
            std::any::type_name_of_val(self)
        ))
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
