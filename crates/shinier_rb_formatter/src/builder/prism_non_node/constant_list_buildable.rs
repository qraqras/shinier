use crate::builder::builder::{array, none};
use crate::builder::{Buildable, BuildableList};
use crate::document::Document;
use ruby_prism::ConstantList;

impl<'a> Buildable<'_> for ConstantList<'_> {
    fn build(&self) -> Document {
        unimplemented!("ConstantList");
    }
}

impl<'a> BuildableList<'_> for ConstantList<'_> {
    fn build(&self, separator: Document) -> Document {
        if self.iter().next().is_none() {
            return none();
        }
        let mut vec = Vec::new();
        for (i, node) in self.iter().enumerate() {
            if i > 0 {
                vec.push(separator.clone());
            }
            vec.push(node.build());
        }
        array(&vec)
    }
}
