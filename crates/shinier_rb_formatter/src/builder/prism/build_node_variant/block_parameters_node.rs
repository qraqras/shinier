use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::build_node::build_nodelist;
use ruby_prism::BlockParametersNode;
use ruby_prism::NodeList;
use ruby_prism::ParametersNode;

/// Builds BlockParametersNode.
///
/// Parameters and Locals are separated by semicolon if both exist.
pub fn build_block_parameters_node(node: &BlockParametersNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let parameters = node.parameters();
    let locals = node.locals();
    let opening_loc = node.opening_loc();
    let closing_loc = node.closing_loc();

    let parameters = _build_parameters(parameters, locals, ctx);
    match (opening_loc, closing_loc) {
        (Some(o), Some(c)) => group(array(&[
            build_location(o, ctx),
            indent(array(&[softline(), parameters])),
            softline(),
            build_location(c, ctx),
        ])),
        (None, None) => group(parameters),
        _ => unreachable!(),
    }
}

/// Helper function to build parameters and locals with semicolon separation.
fn _build_parameters(
    parameters: Option<ParametersNode<'_>>,
    locals: NodeList<'_>,
    ctx: &mut BuildContext,
) -> Option<Document> {
    match parameters {
        Some(p) => {
            let parameters_doc = build_node(p.as_node(), ctx);
            let locals_doc = build_nodelist(locals, ctx);
            match locals_doc.is_empty() {
                true => parameters_doc,
                false => {
                    let mut docs = Vec::new();
                    docs.push(parameters_doc);
                    docs.push(string("; "));
                    docs.extend(locals_doc);
                    array(&docs)
                }
            }
        }
        None => array(&build_nodelist(locals, ctx)),
    }
}
