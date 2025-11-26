use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::WhileNode;

/// Builds WhileNode.
///
/// Formats styles:
/// - Basic While Style: whlie ... end
/// - Inline While Style: ... while ...
/// - Begin Modifier While Style: begin ... while ...
///
/// Always omits 'do' keyword if present.
pub fn build_while_node(node: &WhileNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let is_begin_modifier = node.is_begin_modifier();
    let keyword_loc = node.keyword_loc();
    let _do_keyword_loc = node.do_keyword_loc();
    let closing_loc = node.closing_loc();
    let predicate = node.predicate();
    let statements = node.statements();

    match is_begin_modifier {
        // Begin Modifier While
        true => group(array(&[
            statements.map(|s| build_node(s.as_node(), ctx)).flatten(),
            space(),
            build_location(keyword_loc, ctx),
            indent(group(array(&[line(), build_node(predicate, ctx)]))),
        ])),
        false => match closing_loc {
            // Basic While
            Some(c) => {
                // basic while
                group(array(&[
                    build_location(keyword_loc, ctx),
                    indent(group(array(&[line(), build_node(predicate, ctx)]))),
                    indent(group(array(&[statements
                        .map(|s| array(&[hardline(), build_node(s.as_node(), ctx)]))
                        .flatten()]))),
                    hardline(),
                    build_location(c, ctx),
                ]))
            }
            // Inline While
            None => {
                // inline while
                group(array(&[
                    statements.map(|s| build_node(s.as_node(), ctx)).flatten(),
                    space(),
                    build_location(keyword_loc, ctx),
                    indent(group(array(&[line(), build_node(predicate, ctx)]))),
                ]))
            }
        },
    }
}
