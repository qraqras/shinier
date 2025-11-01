use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, line, none, space, string};
use crate::builder::prism::helper::owning_comments;
use crate::document::Document;
use crate::keyword::ALIAS;
use ruby_prism::AliasGlobalVariableNode;

impl<'sh> Build for AliasGlobalVariableNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &AliasGlobalVariableNode, context: &mut BuildContext) -> Document {
    let old_name = node.old_name();
    let new_name = node.new_name();
    group(array(&[
        string(ALIAS),
        indent(group(array(&[
            line(),
            match owning_comments(&node.as_node(), context) {
                Some(comment) => array(&[comment, hardline()]),
                None => none(),
            },
            group(array(&[
                old_name.build(context),
                space(),
                new_name.build(context),
            ])),
        ]))),
    ]))
}
