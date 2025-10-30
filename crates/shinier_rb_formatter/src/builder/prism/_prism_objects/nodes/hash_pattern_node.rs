use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, group, indent, line, softline, string};
use crate::document::Document;
use crate::helper::separate_docs::separate_docs;
use crate::keyword::{BRACES, BRACKETS, COMMA};
use ruby_prism::HashPatternNode;

impl<'sh> Build for HashPatternNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &HashPatternNode, context: &mut BuildContext) -> Document {
    let constant = node.constant();
    let elements = node.elements();
    let rest = node.rest();

    let separator = array(&[string(COMMA), line()]);

    match constant {
        Some(constant) => group(array(&[
            constant.build(context),
            string(BRACKETS.0),
            indent(array(&[
                softline(),
                array(&separate_docs(
                    &[elements.build(context, &separator), rest.build(context)],
                    separator.clone(),
                )),
            ])),
            softline(),
            string(BRACKETS.1),
        ])),
        None => group(array(&[
            string(BRACES.0),
            indent(array(&[
                line(),
                array(&separate_docs(
                    &[elements.build(context, &separator), rest.build(context)],
                    separator.clone(),
                )),
            ])),
            line(),
            string(BRACES.1),
        ])),
    }
}
