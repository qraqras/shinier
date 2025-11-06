use crate::Build;
use crate::BuildContext;
use crate::builder::builder::array;
use crate::builder::builder::group;
use crate::builder::builder::hardline;
use crate::builder::builder::indent;
use crate::builder::builder::line;
use crate::builder::builder::none;
use crate::builder::builder::space;
use crate::builder::builder::string;
use crate::builder::prism::helper::owning_comments_with;
use crate::document::Document;
use crate::keyword::ROCKET;
use ruby_prism::AssocNode;

impl<'sh> Build for AssocNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let key = self.key();
        let value = self.value();
        group(array(&[
            key.build(context),
            space(),
            string(ROCKET),
            indent(array(&[
                owning_comments_with(&self.as_node(), context, Some(hardline()), None)
                    .unwrap_or(none()),
                line(),
                value.build(context),
            ])),
        ]))
    }
}
