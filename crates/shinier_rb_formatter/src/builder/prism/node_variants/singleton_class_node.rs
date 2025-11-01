use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{CLASS, END, SINGLETON};
use ruby_prism::SingletonClassNode;

impl<'sh> Build for SingletonClassNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &SingletonClassNode, context: &mut BuildContext) -> Document {
    let expression = node.expression();
    let body = node.body();
    group(array(&[
        string(CLASS),
        space(),
        string(SINGLETON),
        space(),
        expression.build(context),
        indent(body.build_with(context, Some(hardline()), None)),
        line(),
        string(END),
    ]))
}
