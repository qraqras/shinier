use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, group, line, space, string};
use crate::document::Document;
use crate::keyword::{COMMA, UNDEF};
use ruby_prism::UndefNode;

impl<'sh> Build for Option<&UndefNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&UndefNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let names = node.names();
    group(array(&[
        string(UNDEF),
        space(),
        names.build(context, &array(&[string(COMMA), line()])),
    ]))
}
