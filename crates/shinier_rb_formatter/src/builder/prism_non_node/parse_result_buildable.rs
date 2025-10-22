use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::ParseResult;

impl<'a> Buildable<'_> for ParseResult<'_> {
    fn build(&self) -> Document {
        unimplemented!("ParseResult");
    }
}
