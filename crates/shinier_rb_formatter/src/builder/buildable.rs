use crate::builder::builder::*;
use crate::document::Document;

pub trait Buildable<'sh> {
    fn build(&self) -> Document;
    fn build_with(&self, before: Option<Document>, after: Option<Document>) -> Document {
        let before = before.unwrap_or_else(|| none());
        let after = after.unwrap_or_else(|| none());
        array(&[before, self.build(), after])
    }
    fn build_or(&self, _default: Document) -> Document {
        unimplemented!("only implemented for Option<T>")
    }
}

impl<'sh, T: Buildable<'sh>> Buildable<'sh> for Option<T> {
    fn build(&self) -> Document {
        match self {
            Some(s) => s.build(),
            None => none(),
        }
    }
    fn build_with(&self, before: Option<Document>, after: Option<Document>) -> Document {
        match self {
            Some(s) => s.build_with(before, after),
            None => none(),
        }
    }
    fn build_or(&self, default: Document) -> Document {
        match self {
            Some(s) => s.build(),
            None => default,
        }
    }
}

pub trait BuildableList<'sh> {
    fn build(&self, separator: Document) -> Document;
    fn build_with(
        &self,
        separator: Document,
        before: Option<Document>,
        after: Option<Document>,
    ) -> Document {
        let before = before.unwrap_or_else(|| none());
        let after = after.unwrap_or_else(|| none());
        array(&[before, self.build(separator), after])
    }
    fn build_or(&self, _separator: Document, _default: Document) -> Document {
        unimplemented!("only implemented for Option<T>")
    }
}

impl<'sh, T: BuildableList<'sh>> BuildableList<'sh> for Option<T> {
    fn build(&self, separator: Document) -> Document {
        match self {
            Some(s) => s.build(separator),
            None => none(),
        }
    }
    fn build_with(
        &self,
        separator: Document,
        before: Option<Document>,
        after: Option<Document>,
    ) -> Document {
        match self {
            Some(s) => s.build_with(separator, before, after),
            None => none(),
        }
    }
    fn build_or(&self, separator: Document, default: Document) -> Document {
        match self {
            Some(s) => s.build(separator),
            None => default,
        }
    }
}

// &[u8]
impl<'sh> Buildable<'sh> for &[u8] {
    fn build(&self) -> Document {
        match std::str::from_utf8(self) {
            Ok(s) => string(s),
            Err(_) => string("<invalid utf8>"),
        }
    }
}
