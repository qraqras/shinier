use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, group, line, space, string};
use crate::document::Document;
use crate::keyword::{COMMA, UNDEF};
use ruby_prism::UndefNode;

impl<'sh> Build for UndefNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &UndefNode, context: &mut BuildContext) -> Document {
    let names = node.names();
    group(array(&[
        string(UNDEF),
        space(),
        names.build(context, &array(&[string(COMMA), line()])),
    ]))
}
