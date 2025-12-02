use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::builder_helper::*;
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

    let class_keyword_end_offset = class_keyword_loc.end_offset();
    let constant_path_end_offset = constant_path.location().end_offset();

    let inheritance_part = match (inheritance_operator_loc, superclass) {
        (Some(i), Some(s)) => {
            let inheritance_operator_loc_end_offset = i.end_offset();
            array(&[
                line_if_has_comments(constant_path_end_offset, i.end_offset(), ctx),
                build_location(i, ctx),
                indent(array(&[
                    line_if_has_comments(inheritance_operator_loc_end_offset, s.location().start_offset(), ctx),
                    build_node(s, ctx),
                ])),
            ])
        }
        (None, None) => None,
        _ => unreachable!("inheritance_operator_loc and superclass should be both Some or None"),
    };

    group(array(&[
        build_location(class_keyword_loc, ctx),
        indent(array(&[
            line_if_has_comments(class_keyword_end_offset, constant_path.location().start_offset(), ctx),
            build_node(constant_path, ctx),
            inheritance_part,
        ])),
        indent(body.map(|b| group(array(&[hardline(), build_node(b, ctx)]))).flatten()),
        hardline(),
        build_location(end_keyword_loc, ctx),
    ]))
}
