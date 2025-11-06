use crate::Build;
use crate::BuildContext;
use crate::NodeVariant;
use crate::builder::builder::array;
use crate::builder::builder::group;
use crate::builder::builder::hardline;
use crate::builder::builder::indent;
use crate::builder::builder::none;
use crate::builder::builder::string;
use crate::builder::prism::helper::owning_comments;
use crate::document::Document;
use crate::keyword::BEGIN;
use crate::keyword::END;
use ruby_prism::BeginNode;

impl<'sh> Build for BeginNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let statements = self.statements();
        let rescue_clause = self.rescue_clause();
        let else_clause = self.else_clause();
        let ensure_clause = self.ensure_clause();
        group(array(&[
            string(BEGIN),
            indent(array(&[statements.build_with(
                context,
                Some(hardline()),
                None,
            )])),
            match rescue_clause {
                Some(rescue) => array(&[hardline(), rescue.as_node().build(context)]),
                None => none(),
            },
            else_clause.build_with(context, Some(hardline()), None),
            ensure_clause.build_with(context, Some(hardline()), None),
            indent(match owning_comments(&self.as_node(), context) {
                Some(comments) => array(&[hardline(), comments]),
                None => none(),
            }),
            hardline(),
            string(END),
        ]))
    }
}
