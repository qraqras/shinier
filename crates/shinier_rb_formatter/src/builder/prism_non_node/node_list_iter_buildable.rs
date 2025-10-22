use crate::builder::Buildable;
use crate::document::Document;
use ruby_prism::NodeListIter;

impl<'a> Buildable<'_> for NodeListIter<'_> {
    fn build(&self) -> Document {
        unimplemented!("NodeListIter");
    }
}
