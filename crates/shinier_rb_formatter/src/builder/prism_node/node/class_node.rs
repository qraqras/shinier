use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, hardline, indent, space, string};
use crate::document::Document;
use crate::keyword::{CLASS, END, INHERITES};
use ruby_prism::ClassNode;

pub fn build_node(node: Option<&ClassNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let constant_path = node.constant_path();
    let super_class = node.superclass();
    let body = node.body();
    group(array(&[
        string(CLASS),
        space(),
        constant_path.build(context),
        super_class.build_with(
            context,
            Some(array(&[space(), string(INHERITES), space()])),
            Some(space()),
        ),
        indent(body.build_with(context, Some(hardline()), None)),
        hardline(),
        string(END),
    ]))
}
