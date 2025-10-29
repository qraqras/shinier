use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, group, indent, line, softline, string};
use crate::builder::helper::separate_docs::separate_docs;
use crate::document::Document;
use crate::keyword::{COMMA, PARENTHESES};
use ruby_prism::MultiTargetNode;

impl<'sh> Build for Option<&MultiTargetNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&MultiTargetNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let lefts = node.lefts();
    let rest = node.rest();
    let rights = node.rights();

    let separator = array(&[string(COMMA), line()]);
    group(array(&[
        string(PARENTHESES.0),
        indent(array(&[
            softline(),
            group(array(&separate_docs(
                &[
                    lefts.build(context, &separator),
                    rest.build(context),
                    rights.build(context, &separator),
                ],
                separator.clone(),
            ))),
        ])),
        softline(),
        string(PARENTHESES.1),
    ]))
}
