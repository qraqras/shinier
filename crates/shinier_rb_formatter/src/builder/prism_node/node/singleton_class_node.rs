use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{CLASS, END, SINGLETON};
use ruby_prism::SingletonClassNode;

pub fn build_node(node: Option<&SingletonClassNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
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
