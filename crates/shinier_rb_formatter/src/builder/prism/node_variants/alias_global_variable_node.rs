use crate::Build;
use crate::BuildContext;
use crate::builder::builder::array;
use crate::builder::builder::conditional_group;
use crate::builder::builder::hardline;
use crate::builder::builder::indent;
use crate::builder::builder::line;
use crate::builder::builder::space;
use crate::builder::builder::string;
use crate::builder::prism::helper::owning_comments;
use crate::document::Document;
use crate::keyword::ALIAS;
use ruby_prism::AliasGlobalVariableNode;

impl<'sh> Build for AliasGlobalVariableNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let old_name = self.old_name();
        let new_name = self.new_name();
        conditional_group(&[
            array(&[
                string(ALIAS),
                indent(array(&[
                    match owning_comments(&self.as_node(), context) {
                        Some(comment) => array(&[hardline(), comment, hardline()]),
                        None => space(),
                    },
                    old_name.build(context),
                    space(),
                    new_name.build(context),
                ])),
            ]),
            array(&[
                string(ALIAS),
                indent(array(&[
                    match owning_comments(&self.as_node(), context) {
                        Some(comment) => array(&[hardline(), comment, hardline()]),
                        None => line(),
                    },
                    old_name.build(context),
                    space(),
                    new_name.build(context),
                ])),
            ]),
        ])
    }
}
