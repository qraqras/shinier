use crate::builder::Buildable;
use crate::doc::{Doc, text};
use ruby_prism::Comments;

impl<'a> Buildable<'_> for Comments<'_> {
    fn build(&self) -> Doc {
        text(format!(
            "not implemented: {:?}",
            std::any::type_name_of_val(self)
        ))
    }
}
