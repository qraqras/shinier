use crate::builder::Buildable;
use crate::doc::Doc;
use ruby_prism::NodeListIter;

impl<'a> Buildable<'_> for NodeListIter<'_> {
    fn build(&self) -> Doc {
        unimplemented!("NodeListIter");
    }
}
