use crate::builder::{Buildable, BuildableList};
use crate::doc::Doc;
use ruby_prism::NodeList;

impl<'a> BuildableList<'_> for NodeList<'_> {
    fn build<F: Fn(&[Doc]) -> Doc>(&self, separator: Doc, wrapper: F) -> Doc {
        let mut vec = Vec::new();
        for (i, node) in self.iter().enumerate() {
            if i > 0 {
                vec.push(separator.clone());
            }
            vec.push(node.build());
        }
        wrapper(&vec)
    }
}
