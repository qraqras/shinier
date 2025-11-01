use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::{ALIAS_METHOD, COMMA};
use ruby_prism::AliasMethodNode;

impl<'sh> Build for AliasMethodNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &AliasMethodNode, context: &mut BuildContext) -> Document {
    let old_name = node.old_name();
    let new_name = node.new_name();
    group(array(&[
        string(ALIAS_METHOD),
        space(),
        new_name.build(context),
        string(COMMA),
        space(),
        old_name.build(context),
    ]))
}
