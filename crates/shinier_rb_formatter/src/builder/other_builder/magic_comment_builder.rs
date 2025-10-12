use crate::builder::Buildable;
use crate::doc::{Doc, text};
use ruby_prism::MagicComment;

impl<'a> Buildable<'_> for MagicComment<'_> {
    fn build(&self) -> Doc {
        text(format!(
            "not implemented: {:?}",
            std::any::type_name_of_val(self)
        ))
    }
}
