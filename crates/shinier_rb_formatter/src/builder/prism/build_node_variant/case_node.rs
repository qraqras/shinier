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

    let break_if_comments = match &predicate {
        Some(p) => line_if_has_comments(case_keyword_loc.end_offset(), p.location().start_offset(), ctx),
        None => None,
    };

    let mut condition_docs = Vec::new();
    for condition in conditions.iter() {
        condition_docs.push(hardline());
        condition_docs.push(build_node(condition, ctx));
    }

    group(array(&[
        build_location(case_keyword_loc, ctx),
        predicate
            .map(|p| indent(array(&[break_if_comments, build_node(p, ctx)])))
            .flatten(),
        array(&condition_docs),
        else_clause
            .map(|e| array(&[hardline(), build_node(e.as_node(), ctx)]))
            .flatten(),
        hardline(),
        build_location(end_keyword_loc, ctx),
    ]))
}
