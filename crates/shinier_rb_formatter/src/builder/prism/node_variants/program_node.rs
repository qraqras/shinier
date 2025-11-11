use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, dedent_to_root, hardline, literalline, none};
use crate::builder::prism::helper::dangling_comments;
use crate::document::Document;
use ruby_prism::ProgramNode;

impl<'sh> Build for ProgramNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let statements = self.statements();
        array(&[
            statements.build(context),
            match dangling_comments(context) {
                Some(comments) => dedent_to_root(array(&[hardline(), comments])),
                None => none(),
            },
            literalline(),
        ])
    }
}
