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
    let constant_path_start_offset = constant_path.location().start_offset();
    let constant_path_end_offset = constant_path.location().end_offset();
    let inheritance_operator_start_offset = inheritance_operator_loc.as_ref().map(|loc| loc.start_offset());
    let inheritance_operator_end_offset = inheritance_operator_loc.as_ref().map(|loc| loc.end_offset());
    let superclass_start_offset = superclass.as_ref().map(|s| s.location().start_offset());

    let after_class_keyword = |t: Option<Document>, f: Option<Document>, ctx: &mut BuildContext<'_>| {
        if_has_comments_beween(t, f, class_keyword_end_offset, constant_path_start_offset, ctx)
    };
    let after_constant_path = |t: Option<Document>, f: Option<Document>, ctx: &mut BuildContext<'_>| {
        match inheritance_operator_start_offset {
            Some(i) => if_has_comments_beween(t, f, constant_path_end_offset, i, ctx),
            None => None,
        }
    };
    let after_inheritance_operator = |t: Option<Document>, f: Option<Document>, ctx: &mut BuildContext<'_>| match (
        inheritance_operator_end_offset,
        superclass_start_offset,
    ) {
        (Some(i), Some(s)) => if_has_comments_beween(t, f, i, s, ctx),
        _ => None,
    };

    let class_keyword_loc_doc = build_location(class_keyword_loc, ctx);
    let constant_path_doc = build_node(constant_path, ctx);
    let inheritance_operator_loc_doc = inheritance_operator_loc.map(|loc| build_location(loc, ctx)).flatten();
    let superclass_doc = superclass.map(|s| build_node(s, ctx)).flatten();

    let opening = conditional_group(&[
        group(array(&[
            //
            class_keyword_loc_doc.clone(),
            indent(array(&[
                after_class_keyword(hardline(), space(), ctx),
                constant_path_doc.clone(),
                indent(array(&[
                    after_constant_path(hardline(), space(), ctx),
                    inheritance_operator_loc_doc.clone(),
                ])),
                indent(array(&[
                    after_inheritance_operator(hardline(), space(), ctx),
                    superclass_doc.clone(),
                ])),
            ])),
        ])),
        group(array(&[
            //
            class_keyword_loc_doc.clone(),
            indent(array(&[
                after_class_keyword(hardline(), space(), ctx),
                constant_path_doc.clone(),
                indent(array(&[
                    after_constant_path(hardline(), space(), ctx),
                    inheritance_operator_loc_doc.clone(),
                ])),
                indent(array(&[
                    after_inheritance_operator(hardline(), hardline(), ctx),
                    superclass_doc.clone(),
                ])),
            ])),
        ])),
        group(array(&[
            //
            class_keyword_loc_doc.clone(),
            indent(array(&[
                after_class_keyword(hardline(), hardline(), ctx),
                constant_path_doc.clone(),
                indent(array(&[
                    after_constant_path(hardline(), space(), ctx),
                    inheritance_operator_loc_doc.clone(),
                ])),
                indent(array(&[
                    after_inheritance_operator(hardline(), hardline(), ctx),
                    superclass_doc.clone(),
                ])),
            ])),
        ])),
    ]);

    group(array(&[
        opening,
        indent(body.map(|b| group(array(&[hardline(), build_node(b, ctx)]))).flatten()),
        hardline(),
        build_location(end_keyword_loc, ctx),
    ]))
}
