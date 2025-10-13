use crate::builder::Buildable;
use crate::doc::Doc;
use ruby_prism::ParseResult;

impl<'a> Buildable<'_> for ParseResult<'_> {
    fn build(&self) -> Doc {
        unimplemented!("ParseResult");
    }
}
