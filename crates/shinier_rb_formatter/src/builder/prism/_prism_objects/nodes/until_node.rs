use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{END, UNTIL};
use ruby_prism::UntilNode;

impl<'sh> Build for Option<&UntilNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&UntilNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let is_begin_modifier = node.is_begin_modifier();
    let predicate = node.predicate();
    let statements = node.statements();
    match is_begin_modifier {
        true => group(array(&[
            statements.as_ref().build(context),
            space(),
            string(UNTIL),
            space(),
            predicate.build(context),
        ])),
        false => group(array(&[
            string(UNTIL),
            space(),
            predicate.build(context),
            indent(array(&[hardline(), statements.as_ref().build(context)])),
            line(),
            string(END),
        ])),
    }
}
