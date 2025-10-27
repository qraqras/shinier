use crate::builder::builder::{array, none};
use crate::document::Document;
use crate::{BuildContext, BuildPrismNode};
use ruby_prism::NodeList;

pub trait BuildPrismNodeList {
    fn _build(&self, context: &mut BuildContext, separator: &Document) -> Document;
    fn build(&self, context: &mut BuildContext, separator: &Document) -> Document {
        self._build(context, separator)
    }
    fn build_with(
        &self,
        context: &mut BuildContext,
        separator: &Document,
        before: Option<Document>,
        after: Option<Document>,
    ) -> Document {
        let before = before.unwrap_or(Document::None);
        let after = after.unwrap_or(Document::None);
        Document::Array(Vec::from(&[before, self.build(context, separator), after]))
    }
}

impl BuildPrismNodeList for NodeList<'_> {
    fn _build(&self, context: &mut BuildContext, separator: &Document) -> Document {
        if self.iter().next().is_none() {
            return none();
        }
        let mut vec = Vec::new();
        for (i, node) in self.iter().enumerate() {
            if i > 0 {
                vec.push(separator.clone());
            }
            vec.push(node._build(context));
        }
        array(&vec)
    }
}

impl<'a> BuildPrismNodeList for Option<NodeList<'_>> {
    fn _build(&self, context: &mut BuildContext, separator: &Document) -> Document {
        match self {
            Some(node) => node._build(context, separator),
            None => none(),
        }
    }
}
