use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::builder_helper::*;
use ruby_prism::CaseMatchNode;

/// Builds CaseMatchNode.
pub fn build_case_match_node(node: &CaseMatchNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let predicate = node.predicate();
    let conditions = node.conditions();
    let else_clause = node.else_clause();
    let case_keyword_loc = node.case_keyword_loc();
    let end_keyword_loc = node.end_keyword_loc();

    let break_if_comments = match &predicate {
        Some(p) => line_if_has_comments(case_keyword_loc.end_offset(), p.location().start_offset(), ctx),
        None => None,
    };

    let mut conditions_docs = Vec::new();
    for condition in conditions.iter() {
        conditions_docs.push(hardline());
        conditions_docs.push(build_node(condition, ctx));
    }

    group(array(&[
        build_location(case_keyword_loc, ctx),
        predicate
            .map(|p| indent(array(&[break_if_comments, build_node(p, ctx)])))
            .flatten(),
        array(&conditions_docs),
        else_clause
            .map(|n| array(&[hardline(), build_node(n.as_node(), ctx)]))
            .flatten(),
        hardline(),
        build_location(end_keyword_loc, ctx),
    ]))
}
