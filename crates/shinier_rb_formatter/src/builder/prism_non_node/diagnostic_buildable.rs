use crate::builder::Buildable;
use crate::doc::Doc;
use ruby_prism::Diagnostic;

impl<'a> Buildable<'_> for Diagnostic<'_> {
    fn build(&self) -> Doc {
        unimplemented!("Diagnostic");
    }
}
