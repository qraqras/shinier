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
use crate::keyword::COLON;
use crate::keyword::ROCKET;
use ruby_prism::AssocNode;
use ruby_prism::Node;

impl<'sh> Build for AssocNode<'sh> {
    // if the key is a symbol node, we use hash label style
    // ```ruby
    // { key: value } # <- { :key => value }
    // ```
    fn __build__(&self, context: &mut BuildContext) -> Document {
        fn build_key(key: &Node, context: &mut BuildContext) -> Document {
            context.hash_label_style = key.as_symbol_node().is_some();
            let key = key.build(context);
            context.hash_label_style = false;
            key
        }
        let key = self.key();
        let value = self.value();
        group(array(&[
            build_key(&key, context),
            match key.as_symbol_node().is_some() {
                true => string(COLON),
                false => array(&[space(), string(ROCKET)]),
            },
            indent(array(&[
                owning_comments_with(&self.as_node(), context, Some(hardline()), None)
                    .unwrap_or(none()),
                line(),
                value.build(context),
            ])),
        ]))
    }
}
