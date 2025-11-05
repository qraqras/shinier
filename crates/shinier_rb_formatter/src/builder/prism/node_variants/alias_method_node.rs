use crate::Build;
use crate::BuildContext;
use crate::builder::builder::array;
use crate::builder::builder::conditional_group;
use crate::builder::builder::indent;
use crate::builder::builder::line;
use crate::builder::builder::softline;
use crate::builder::builder::space;
use crate::builder::builder::string;
use crate::builder::prism::helper::owning_comments;
use crate::document::Document;
use crate::keyword::ALIAS_METHOD;
use crate::keyword::COMMA;
use crate::keyword::PARENTHESES;
use ruby_prism::AliasMethodNode;

impl<'sh> Build for AliasMethodNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let old_name = self.old_name();
        let new_name = self.new_name();
        owning_comments(&self.as_node(), context); // consume owning comments
        conditional_group(&[
            array(&[
                string(ALIAS_METHOD),
                space(),
                new_name.build(context),
                string(COMMA),
                space(),
                old_name.build(context),
            ]),
            array(&[
                string(ALIAS_METHOD),
                string(PARENTHESES.0),
                indent(array(&[
                    softline(),
                    new_name.build(context),
                    string(COMMA),
                    line(),
                    old_name.build(context),
                ])),
                softline(),
                string(PARENTHESES.1),
            ]),
        ])
    }
}
