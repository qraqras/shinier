use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ClassNode;

/// Builds ClassNode.
///
/// Prefer a two-line format for class definitions with no body.
/// https://github.com/rubocop/ruby-style-guide?tab=readme-ov-file#single-line-classes
pub fn build_class_node(node: &ClassNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let class_keyword_loc = node.class_keyword_loc();
    let constant_path = node.constant_path();
    let inheritance_operator_loc = node.inheritance_operator_loc();
    let superclass = node.superclass();
    let body = node.body();
    let end_keyword_loc = node.end_keyword_loc();

    let inheritance_doc = match (inheritance_operator_loc, superclass) {
        (Some(i), Some(s)) => group(array(&[
            space(),
            build_location(i, ctx),
            indent(array(&[line(), build_node(s, ctx)])),
        ])),
        (None, None) => None,
        _ => unreachable!(),
    };

    group(array(&[
        build_location(class_keyword_loc, ctx),
        indent(group(array(&[line(), build_node(constant_path, ctx), inheritance_doc]))),
        indent(body.map(|b| array(&[hardline(), build_node(b, ctx)])).flatten()),
        hardline(),
        build_location(end_keyword_loc, ctx),
    ]))
}
