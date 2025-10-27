use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::ALIAS;
use ruby_prism::AliasGlobalVariableNode;

pub fn build_node(node: Option<&AliasGlobalVariableNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
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
