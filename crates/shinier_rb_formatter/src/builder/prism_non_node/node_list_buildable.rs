use crate::BuildPrismNode;
use crate::builder::builder::{array, none};
use crate::document::Document;
use ruby_prism::{Comments, NodeList};
use std::collections::HashMap;
use std::iter::Peekable;

pub trait BuildPrismNodeList {
    fn _build(
        &self,
        separator: &Document,
        comments: &mut Peekable<Comments>,
        option: Option<&HashMap<&str, bool>>,
    ) -> Document;
    fn build(&self, separator: &Document, comments: &mut Peekable<Comments>) -> Document {
        self._build(separator, comments, None)
    }
    fn build_with(
        &self,
        separator: &Document,
        comments: &mut Peekable<Comments>,
        before: Option<Document>,
        after: Option<Document>,
    ) -> Document {
        let before = before.unwrap_or(Document::None);
        let after = after.unwrap_or(Document::None);
        Document::Array(Vec::from(&[before, self.build(separator, comments), after]))
    }
}

impl BuildPrismNodeList for NodeList<'_> {
    fn _build(
        &self,
        separator: &Document,
        comments: &mut Peekable<Comments>,
        option: Option<&HashMap<&str, bool>>,
    ) -> Document {
        if self.iter().next().is_none() {
            return none();
        }
        let mut vec = Vec::new();
        for (i, node) in self.iter().enumerate() {
            if i > 0 {
                vec.push(separator.clone());
            }
            vec.push(node._build(comments, option));
        }
        array(&vec)
    }
}

impl<'a> BuildPrismNodeList for Option<NodeList<'_>> {
    fn _build(
        &self,
        separator: &Document,
        comments: &mut Peekable<Comments>,
        option: Option<&HashMap<&str, bool>>,
    ) -> Document {
        match self {
            Some(node) => node._build(separator, comments, option),
            None => none(),
        }
    }
}
