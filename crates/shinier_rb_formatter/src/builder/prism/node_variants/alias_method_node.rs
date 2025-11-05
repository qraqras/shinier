use crate::Build;
use crate::BuildContext;
use crate::builder::builder::array;
use crate::builder::builder::conditional_group;
use crate::builder::builder::hardline;
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
        // build child nodes
        let owning_comments = owning_comments(&self.as_node(), context);
        let old_name = old_name.build(context);
        let new_name = new_name.build(context);
        conditional_group(&[
            array(&[
                match owning_comments.clone() {
                    Some(comment) => array(&[comment, hardline()]),
                    None => none(),
                },
                string(ALIAS_METHOD),
                space(),
                new_name.clone(),
                string(COMMA),
                space(),
                old_name.clone(),
            ]),
            array(&[
                match owning_comments.clone() {
                    Some(comment) => array(&[comment, hardline()]),
                    None => none(),
                },
                string(ALIAS_METHOD),
                owning_comments.clone().unwrap_or(none()),
                string(PARENTHESES.0),
                indent(array(&[
                    softline(),
                    new_name.clone(),
                    string(COMMA),
                    line(),
                    old_name.clone(),
                ])),
                softline(),
                string(PARENTHESES.1),
            ]),
        ])
    }
}
