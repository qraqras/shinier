use crate::builder::{Buildable, BuildableList};
use crate::doc::{Doc, sequence};
use ruby_prism::NodeList;

impl<'a> Buildable<'_> for NodeList<'_> {
    fn build(&self) -> Doc {
        unimplemented!("NodeList");
    }
}

impl<'a> BuildableList<'_> for NodeList<'_> {
    fn build(&self, arg: &Doc) -> Doc {
        let mut vec = Vec::new();
        for (i, node) in self.iter().enumerate() {
            if i > 0 {
                vec.push(arg.clone());
            }
            vec.push(node.build());
        }
        sequence(&vec)
    }
}
