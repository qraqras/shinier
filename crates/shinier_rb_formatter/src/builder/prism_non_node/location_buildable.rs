use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::Location;

impl<'a> Buildable<'_> for Location<'_> {
    fn build(&self) -> Document {
        self.as_slice().build()
    }
}
