use crate::Build;
use crate::BuildContext;
use crate::builder::builder::array;
use crate::builder::builder::group;
use crate::builder::builder::hardline;
use crate::builder::builder::if_break;
use crate::builder::builder::indent;
use crate::builder::builder::line;
use crate::builder::builder::none;
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
        array(&[
            match owning_comments(&self.as_node(), context) {
                Some(comment) => array(&[comment, hardline()]),
                None => none(),
            },
            group(array(&[
                string(ALIAS_METHOD),
                if_break(string(PARENTHESES.0), space(), None),
                indent(array(&[
                    softline(),
                    new_name.build(context),
                    string(COMMA),
                    line(),
                    old_name.build(context),
                ])),
                softline(),
                if_break(string(PARENTHESES.1), none(), None),
            ])),
        ])
    }
}
