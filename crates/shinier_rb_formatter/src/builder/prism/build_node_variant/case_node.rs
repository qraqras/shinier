use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::builder_helper::*;
use ruby_prism::CaseNode;

/// Builds CaseNode.
pub fn build_case_node(node: &CaseNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let predicate = node.predicate();
    let conditions = node.conditions();
    let else_clause = node.else_clause();
    let case_keyword_loc = node.case_keyword_loc();
    let end_keyword_loc = node.end_keyword_loc();

    let case_keyword_end_offset = case_keyword_loc.end_offset();
    let predicate_start_offset = predicate.as_ref().map(|p| p.location().start_offset());

    let case_keyword_loc_doc = build_location(case_keyword_loc, ctx);
    let preddicate_doc = predicate.map(|p| build_node(p, ctx)).flatten();

    let after_case_keyword =
        |t: Option<Document>, f: Option<Document>, ctx: &mut BuildContext<'_>| match predicate_start_offset {
            Some(p) => if_has_comments_beween(t, f, case_keyword_end_offset, p, ctx),
            None => None,
        };

    let case_header = conditional_group(&[
        array(&[
            //
            case_keyword_loc_doc.clone(),
            indent(array(&[
                //
                after_case_keyword(hardline(), space(), ctx),
                preddicate_doc.clone(),
            ])),
        ]),
        array(&[
            //
            case_keyword_loc_doc.clone(),
            space(),
            indent(array(&[
                after_case_keyword(hardline(), hardline(), ctx),
                preddicate_doc.clone(),
            ])),
        ]),
    ]);

    let mut condition_docs = Vec::new();
    for condition in conditions.iter() {
        condition_docs.push(hardline());
        condition_docs.push(build_node(condition, ctx));
    }

    group(array(&[
        case_header,
        array(&condition_docs),
        else_clause
            .map(|e| array(&[hardline(), build_node(e.as_node(), ctx)]))
            .flatten(),
        hardline(),
        build_location(end_keyword_loc, ctx),
    ]))
}
