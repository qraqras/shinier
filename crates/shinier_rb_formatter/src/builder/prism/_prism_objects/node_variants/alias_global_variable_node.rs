use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::ALIAS;
use ruby_prism::AliasGlobalVariableNode;

impl<'sh> Build for AliasGlobalVariableNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &AliasGlobalVariableNode, context: &mut BuildContext) -> Document {
    let old_name = node.old_name();
    let new_name = node.new_name();
    group(array(&[
        string(ALIAS),
        space(),
        old_name.build(context),
        space(),
        new_name.build(context),
    ]))
}
