use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::Diagnostics;

impl<'a> Buildable<'_> for Diagnostics<'_> {
    fn build(&self) -> Document {
        unimplemented!("Diagnostics");
    }
}
