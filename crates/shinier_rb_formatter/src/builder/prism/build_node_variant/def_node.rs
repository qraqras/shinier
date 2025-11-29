use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::DefNode;

/// Builds DefNode.
///
/// Avoid single-line methods.
/// https://github.com/rubocop/ruby-style-guide?tab=readme-ov-file#no-single-line-methods
pub fn build_def_node(node: &DefNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let name_loc = node.name_loc();
    let receiver = node.receiver();
    let parameters = node.parameters();
    let body = node.body();
    let def_keyword_loc = node.def_keyword_loc();
    let operator_loc = node.operator_loc();
    let lparen_loc = node.lparen_loc();
    let rparen_loc = node.rparen_loc();
    let equal_loc = node.equal_loc();
    let end_keyword_loc = node.end_keyword_loc();

    let line_or_none = match (&lparen_loc, &rparen_loc) {
        (Some(_), Some(_)) => softline(),
        (None, None) => None,
        _ => unreachable!(),
    };

    // TODO: TESTING
    group(array(&[
        build_location(def_keyword_loc, ctx),
        space(),
        receiver.map(|n| array(&[space(), build_node(n, ctx)])).flatten(),
        operator_loc.map(|loc| build_location(loc, ctx)).flatten(),
        build_location(name_loc, ctx),
        group(array(&[
            lparen_loc.map(|loc| build_location(loc, ctx)).flatten(),
            parameters
                .map(|n| indent(array(&[line_or_none.clone(), build_node(n.as_node(), ctx)])))
                .flatten(),
            rparen_loc
                .map(|loc| array(&[line_or_none.clone(), conditional_group(&[build_location(loc, ctx)])]))
                .flatten(),
        ])),
        body.map(|n| indent(array(&[softline(), build_node(n, ctx)]))).flatten(),
        equal_loc
            .map(|loc| array(&[softline(), build_location(loc, ctx)]))
            .flatten(),
        end_keyword_loc
            .map(|loc| array(&[hardline(), build_location(loc, ctx)]))
            .flatten(),
    ]))
}
