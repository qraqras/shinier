use crate::BuildPrismNode;
use crate::builder::builder::{none, string};
use crate::document::Document;
use ruby_prism::{Comments, ConstantId};
use std::collections::HashMap;
use std::iter::Peekable;

impl<'a> BuildPrismNode for ConstantId<'_> {
    fn _build(
        &self,
        comments: &mut Peekable<Comments>,
        _option: Option<&HashMap<&str, bool>>,
    ) -> Document {
        string(String::from_utf8(self.as_slice().to_vec()).unwrap())
    }
}

impl<'a> BuildPrismNode for Option<ConstantId<'_>> {
    fn _build(
        &self,
        comments: &mut Peekable<Comments>,
        option: Option<&HashMap<&str, bool>>,
    ) -> Document {
        match self {
            Some(node) => node._build(comments, option),
            None => none(),
        }
    }
}
