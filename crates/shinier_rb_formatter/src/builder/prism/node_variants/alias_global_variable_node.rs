use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{
    array, group, group_no_propagation, hardline, indent, line, none, space, string,
};
use crate::builder::prism::helper::owning_comments;
use crate::document::Document;
use crate::keyword::ALIAS;
use ruby_prism::AliasGlobalVariableNode;

impl<'sh> Build for AliasGlobalVariableNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let old_name = self.old_name();
        let new_name = self.new_name();
        group(array(&[
            string(ALIAS),
            indent(array(&[
                line(),
                match owning_comments(&self.as_node(), context) {
                    Some(comment) => array(&[comment, hardline()]),
                    None => none(),
                },
                group_no_propagation(array(&[
                    old_name.build(context),
                    space(),
                    new_name.build(context),
                ])),
            ])),
        ]))
    }
}
