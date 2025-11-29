use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::build_node::build_nodelist;
use ruby_prism::BlockParametersNode;

/// Builds BlockParametersNode.
///
/// Parameters and Locals are separated by semicolon if both exist.
pub fn build_block_parameters_node(node: &BlockParametersNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let parameters = node.parameters();
    let locals = node.locals();
    let opening_loc = node.opening_loc();
    let closing_loc = node.closing_loc();

    // Combines parameters and locals with semicolon separation.
    let mut parameters_and_locals = Vec::new();
    match (parameters, locals.iter().next()) {
        (Some(p), Some(_)) => {
            parameters_and_locals.push(build_node(p.as_node(), ctx));
            parameters_and_locals.push(softline());
            parameters_and_locals.push(string(";"));
            parameters_and_locals.push(line());
            parameters_and_locals.extend(build_nodelist(locals, ctx));
        }
        (Some(p), None) => {
            parameters_and_locals.push(build_node(p.as_node(), ctx));
        }
        (None, Some(_)) => {
            parameters_and_locals.push(string(";"));
            parameters_and_locals.push(line());
            parameters_and_locals.extend(build_nodelist(locals, ctx));
        }
        (None, None) => {}
    };

    match (opening_loc, closing_loc) {
        (Some(o), Some(c)) => group(array(&[
            build_location(o, ctx),
            indent(array(&[softline(), group(array(&parameters_and_locals))])),
            softline(),
            build_location(c, ctx),
        ])),
        (None, None) => group(array(&parameters_and_locals)),
        _ => unreachable!(),
    }
}
