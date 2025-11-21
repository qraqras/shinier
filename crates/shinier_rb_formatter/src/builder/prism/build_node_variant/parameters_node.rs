use crate::Document;
use crate::builder::builder::*;
use crate::builder::keyword::COMMA;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ParametersNode;

pub fn build_parameters_node(node: &ParametersNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let requireds = node.requireds();
    let optionals = node.optionals();
    let rest = node.rest();
    let posts = node.posts();
    let keywords = node.keywords();
    let keyword_rest = node.keyword_rest();
    let block = node.block();

    let mut parameters = Vec::new();
    for required in requireds.iter() {
        parameters.push(build_node(&required, ctx));
    }
    for optional in optionals.iter() {
        parameters.push(build_node(&optional, ctx));
    }
    rest.map(|r| parameters.push(build_node(&r, ctx)));
    for post in posts.iter() {
        parameters.push(build_node(&post, ctx));
    }
    for keyword in keywords.iter() {
        parameters.push(build_node(&keyword, ctx));
    }
    keyword_rest.map(|kr| parameters.push(build_node(&kr, ctx)));
    block.map(|b| parameters.push(build_node(&b.as_node(), ctx)));

    let mut separated_parameters = Vec::new();
    for (i, param) in parameters.into_iter().enumerate() {
        if i > 0 {
            separated_parameters.push(string(COMMA));
            separated_parameters.push(line());
        }
        separated_parameters.push(param);
    }

    // TODO: TESTING
    group(array(&separated_parameters))
}
