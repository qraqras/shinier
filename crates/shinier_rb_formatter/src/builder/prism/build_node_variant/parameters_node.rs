use crate::Document;
use crate::builder::builder::*;
use crate::builder::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::build_node::build_nodelist;
use ruby_prism::ParametersNode;

pub fn build_parameters_node(node: &ParametersNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let requireds = node.requireds();
    let optionals = node.optionals();
    let rest = node.rest();
    let posts = node.posts();
    let keywords = node.keywords();
    let keyword_rest = node.keyword_rest();
    let block = node.block();

    // Collects all parameter documents
    let mut parameters_doc = Vec::new();
    parameters_doc.extend(build_nodelist(requireds, ctx));
    parameters_doc.extend(build_nodelist(optionals, ctx));
    rest.map(|r| parameters_doc.push(build_node(r, ctx)));
    parameters_doc.extend(build_nodelist(posts, ctx));
    parameters_doc.extend(build_nodelist(keywords, ctx));
    keyword_rest.map(|k| parameters_doc.push(build_node(k, ctx)));
    block.map(|b| parameters_doc.push(build_node(b.as_node(), ctx)));

    // Separate parameters with commas and line breaks
    let mut separated_parameters = Vec::new();
    for (i, param) in parameters_doc.into_iter().enumerate() {
        if i > 0 {
            separated_parameters.push(comma());
            separated_parameters.push(line());
        }
        separated_parameters.push(param);
    }

    array(&separated_parameters)
}
