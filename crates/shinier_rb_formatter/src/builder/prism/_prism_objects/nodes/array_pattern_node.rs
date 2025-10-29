use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, group, indent, line, softline, string};
use crate::document::Document;
use crate::helper::separate_docs::separate_docs;
use crate::keyword::{BRACKETS, COMMA};
use ruby_prism::ArrayPatternNode;

impl<'sh> Build for Option<&ArrayPatternNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ArrayPatternNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let constant = node.constant();
    let requireds = node.requireds();
    let rest = node.rest();
    let posts = node.posts();

    let separator = array(&[string(COMMA), line()]);

    let separated_requireds = requireds.build(context, &separator);
    let separated_posts = posts.build(context, &separator);

    group(array(&[
        constant.build(context),
        string(BRACKETS.0),
        indent(array(&[
            softline(),
            group(array(&separate_docs(
                &[separated_requireds, rest.build(context), separated_posts],
                separator.clone(),
            ))),
        ])),
        softline(),
        string(BRACKETS.1),
    ]))
}
