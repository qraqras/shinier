use crate::BuildContext;
use crate::BuildPrismNodeList;
use crate::builder::builder::{array, line, string};
use crate::document::Document;
use crate::keyword::COMMA;
use ruby_prism::KeywordHashNode;

pub fn build_node(node: Option<&KeywordHashNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let elements = node.elements();
    elements.build(context, &array(&[string(COMMA), line()]))
}
