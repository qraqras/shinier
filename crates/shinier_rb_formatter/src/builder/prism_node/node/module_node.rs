use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{END, MODULE};
use ruby_prism::ModuleNode;

impl<'sh> Build for Option<&ModuleNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ModuleNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let constant_path = node.constant_path();
    let body = node.body();
    group(array(&[
        string(MODULE),
        space(),
        constant_path.build(context),
        indent(body.build_with(context, Some(hardline()), None)),
        line(),
        string(END),
    ]))
}
