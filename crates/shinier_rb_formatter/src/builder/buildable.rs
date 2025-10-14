use crate::doc::{Doc, fill, group, text};

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

// &[u8]
impl<'sh> Buildable<'sh> for &[u8] {
    fn build(&self) -> Doc {
        match std::str::from_utf8(self) {
            Ok(s) => text(s),
            Err(_) => text("<invalid utf8>"),
        }
    }
}
