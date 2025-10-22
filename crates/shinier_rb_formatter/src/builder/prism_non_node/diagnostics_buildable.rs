use crate::builder::Buildable;
use crate::document::Doc;
use ruby_prism::Diagnostics;

impl<'a> Buildable<'_> for Diagnostics<'_> {
    fn build(&self) -> Doc {
        unimplemented!("Diagnostics");
    }
}
