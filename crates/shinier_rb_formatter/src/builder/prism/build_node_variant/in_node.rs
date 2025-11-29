use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::builder_helper::*;
use ruby_prism::InNode;

pub fn build_in_node(node: &InNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let pattern = node.pattern();
    let statements = node.statements();
    let in_loc = node.in_loc();
    let _then_loc = node.then_loc();

    let in_loc_end_offset = in_loc.end_offset();
    let pattern_start_offset = pattern.location().start_offset();
    let in_loc_doc = build_location(in_loc, ctx);
    let pattern_doc = build_node(pattern, ctx);
    let after_in = |t: Option<Document>, f: Option<Document>, ctx: &mut BuildContext<'_>| {
        if_has_comments_beween(t, f, in_loc_end_offset, pattern_start_offset, ctx)
    };

    let in_header = conditional_group(&[
        array(&[
            //
            in_loc_doc.clone(),
            indent(array(&[
                //
                after_in(hardline(), space(), ctx),
                pattern_doc.clone(),
            ])),
        ]),
        array(&[
            in_loc_doc.clone(),
            indent(array(&[
                //
                after_in(hardline(), hardline(), ctx),
                pattern_doc.clone(),
            ])),
        ]),
    ]);

    group(array(&[
        in_header,
        statements
            .map(|s| indent(array(&[hardline(), build_node(s.as_node(), ctx)])))
            .flatten(),
    ]))
}
