use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, group, indent, line, string};
use crate::builder::prism::helper::layout::separate_docs;
use crate::document::Document;
use crate::keyword::{BRACKETS, COMMA, PARENTHESES};
use ruby_prism::FindPatternNode;

impl<'sh> Build for FindPatternNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &FindPatternNode, context: &mut BuildContext) -> Document {
    let constant = node.constant();
    let left = node.left();
    let requireds = node.requireds();
    let right = node.right();

    let separator = array(&[string(COMMA), line()]);
    let elements = separate_docs(
        &[
            left.as_node().build(context),
            requireds.build(context, &separator),
            right.build(context),
        ],
        separator.clone(),
    );
    match constant {
        Some(constant) => group(array(&[
            constant.build(context),
            string(PARENTHESES.0),
            indent(array(&[group(array(&elements))])),
            string(PARENTHESES.1),
        ])),
        None => group(array(&[
            string(BRACKETS.0),
            indent(array(&[group(array(&elements))])),
            string(BRACKETS.1),
        ])),
    }
}
