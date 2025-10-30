use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, line, string};
use crate::document::Document;
use crate::keyword::COMMA;
use ruby_prism::KeywordHashNode;

impl<'sh> Build for KeywordHashNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &KeywordHashNode, context: &mut BuildContext) -> Document {
    let elements = node.elements();
    elements.build(context, &array(&[string(COMMA), line()]))
}
