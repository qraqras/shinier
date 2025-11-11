use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::array;
use crate::builder::builder::group;
use crate::builder::builder::hardline;
use crate::builder::builder::indent;
use crate::builder::builder::line;
use crate::builder::builder::none;
use crate::builder::builder::softline;
use crate::builder::builder::string;
use crate::builder::prism::helper::owning_comments;
use crate::document::Document;
use crate::keyword::BRACES;
use crate::keyword::COMMA;
use ruby_prism::HashNode;

impl<'sh> Build for HashNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let elements = self.elements();
        // if empty hash, no spaces inside braces
        let is_blank = elements.iter().next().is_none();
        let interior_line = match is_blank {
            true => softline(),
            false => line(),
        };
        group(array(&[
            string(BRACES.0),
            indent(array(&[
                interior_line.clone(),
                elements.build(context, &array(&[string(COMMA), line()])),
            ])),
            match owning_comments(&self.as_node(), context) {
                Some(comments) => array(&[hardline(), comments]),
                None => none(),
            },
            interior_line.clone(),
            string(BRACES.1),
        ]))
    }
}
