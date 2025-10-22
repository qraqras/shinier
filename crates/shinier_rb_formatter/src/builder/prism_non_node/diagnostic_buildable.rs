use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::Diagnostic;

impl<'a> Buildable<'_> for Diagnostic<'_> {
    fn build(&self) -> Document {
        unimplemented!("Diagnostic");
    }
}
