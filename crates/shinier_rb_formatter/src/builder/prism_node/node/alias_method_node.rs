use crate::builder::builder::*;
use crate::document::*;
use crate::helper::build_symbol_without_colon::build_symbol_without_colon;
use crate::keyword::ALIAS;
use ruby_prism::AliasMethodNode;

pub fn build_node(node: Option<&AliasMethodNode>) -> Doc {
    let node = node.unwrap();
    let old_name = node.old_name();
    let new_name = node.new_name();

    group(array(&[
        string(ALIAS),
        line(),
        indent(array(&[
            build_symbol_without_colon(&new_name),
            space(),
            build_symbol_without_colon(&old_name),
        ])),
    ]))
}
