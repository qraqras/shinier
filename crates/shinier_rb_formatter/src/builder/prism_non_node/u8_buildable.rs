use crate::buildable::Buildable;
use crate::doc::{Doc, text};

impl<'sh> Buildable<'sh> for &[u8] {
    fn build(&self) -> Doc {
        match std::str::from_utf8(self) {
            Ok(s) => text(s),
            Err(_) => text("<invalid utf8>"),
        }
    }
}
