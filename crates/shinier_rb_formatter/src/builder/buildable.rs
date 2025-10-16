use crate::doc::{Doc, fill, group, none, sequence, text};

pub trait Buildable<'sh> {
    fn build(&self) -> Doc;
    fn build_with(&self, before: Option<Doc>, after: Option<Doc>) -> Doc {
        let before = before.unwrap_or_else(|| none());
        let after = after.unwrap_or_else(|| none());
        sequence(&[before, self.build(), after])
    }
    fn build_or(&self, _default: Doc) -> Doc {
        unimplemented!("only implemented for Option<T>")
    }
}

impl<'sh, T: Buildable<'sh>> Buildable<'sh> for Option<T> {
    fn build(&self) -> Doc {
        match self {
            Some(s) => s.build(),
            None => none(),
        }
    }
    fn build_with(&self, before: Option<Doc>, after: Option<Doc>) -> Doc {
        match self {
            Some(s) => s.build_with(before, after),
            None => none(),
        }
    }
    fn build_or(&self, default: Doc) -> Doc {
        match self {
            Some(s) => s.build(),
            None => default,
        }
    }
}

pub trait BuildableList<'sh> {
    fn build<F: Fn(&[Doc]) -> Doc>(&self, separator: Doc, build_function: F) -> Doc;
    fn build_with<F: Fn(&[Doc]) -> Doc>(
        &self,
        separator: Doc,
        build_function: F,
        before: Option<Doc>,
        after: Option<Doc>,
    ) -> Doc {
        let before = before.unwrap_or_else(|| none());
        let after = after.unwrap_or_else(|| none());
        sequence(&[before, self.build(separator, build_function), after])
    }
    fn build_or<F: Fn(&[Doc]) -> Doc>(
        &self,
        _separator: Doc,
        _build_function: F,
        _default: Doc,
    ) -> Doc {
        unimplemented!("only implemented for Option<T>")
    }
}

impl<'sh, T: BuildableList<'sh>> BuildableList<'sh> for Option<T> {
    fn build<F: Fn(&[Doc]) -> Doc>(&self, separator: Doc, build_function: F) -> Doc {
        match self {
            Some(s) => s.build(separator, build_function),
            None => none(),
        }
    }
    fn build_with<F: Fn(&[Doc]) -> Doc>(
        &self,
        separator: Doc,
        build_function: F,
        before: Option<Doc>,
        after: Option<Doc>,
    ) -> Doc {
        match self {
            Some(s) => s.build_with(separator, build_function, before, after),
            None => none(),
        }
    }
    fn build_or<F: Fn(&[Doc]) -> Doc>(
        &self,
        separator: Doc,
        build_function: F,
        default: Doc,
    ) -> Doc {
        match self {
            Some(s) => s.build(separator, build_function),
            None => default,
        }
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
