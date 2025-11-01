use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, group_no_propagation, indent, line, space, string};
use crate::builder::prism::helper::owning_comments;
use crate::document::Document;
use crate::keyword::{ALIAS_METHOD, COMMA};
use ruby_prism::AliasMethodNode;

impl<'sh> Build for AliasMethodNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let old_name = self.old_name();
        let new_name = self.new_name();
        // consume owning comments
        owning_comments(&self.as_node(), context);
        group(array(&[
            string(ALIAS_METHOD),
            indent(array(&[
                line(),
                group_no_propagation(array(&[
                    new_name.build(context),
                    string(COMMA),
                    space(),
                    old_name.build(context),
                ])),
            ])),
        ]))
    }
}
