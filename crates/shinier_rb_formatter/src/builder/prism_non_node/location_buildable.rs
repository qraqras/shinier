use crate::builder::Buildable;
use crate::doc::Doc;
use ruby_prism::Location;

impl<'a> Buildable<'_> for Location<'_> {
    fn build(&self) -> Doc {
        self.as_slice().build()
    }
}
