use crate::doc::{Doc, fill, group};

pub trait Buildable<'sh> {
    fn build(&self) -> Doc;
}

pub trait BuildableList<'sh> {
    fn build(&self, separator: &Doc) -> Doc;
    fn build_group(&self, separator: &Doc) -> Doc {
        group(&[self.build(separator)])
    }
    fn build_fill(&self, separator: &Doc) -> Doc {
        fill(&[self.build(separator)])
    }
}
