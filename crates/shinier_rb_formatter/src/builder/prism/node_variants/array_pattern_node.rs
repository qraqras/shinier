use crate::Build;
use crate::BuildContext;
use crate::builder::builder::array;
use crate::builder::builder::group;
use crate::builder::builder::indent;
use crate::builder::builder::line;
use crate::builder::builder::softline;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::BRACKETS;
use crate::keyword::COMMA;
use ruby_prism::ArrayPatternNode;
use ruby_prism::Node;
use ruby_prism::NodeList;

impl<'sh> Build for ArrayPatternNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let constant = self.constant();
        let requireds = self.requireds();
        let rest = self.rest();
        let posts = self.posts();
        group(array(&[
            constant.build(context),
            string(BRACKETS.0),
            indent(array(&[
                softline(),
                group(array(&build_array_pattern_elements(
                    &requireds, rest, &posts, context,
                ))),
            ])),
            softline(),
            string(BRACKETS.1),
        ]))
    }
}

fn build_array_pattern_elements(
    requireds: &NodeList,
    rest: Option<Node>,
    posts: &NodeList,
    context: &mut BuildContext,
) -> Vec<Document> {
    // collect all nodes in order
    let mut nodes = Vec::new();
    for required in requireds.iter() {
        nodes.push(required);
    }
    if let Some(rest) = rest {
        nodes.push(rest);
    }
    for post in posts.iter() {
        nodes.push(post);
    }
    // build documents with proper commas
    let mut documents = Vec::new();
    for (i, node) in nodes.iter().enumerate() {
        let should_comma = match node {
            Node::ImplicitRestNode { .. } => false,
            _ => true,
        };
        if i > 0 && should_comma {
            documents.push(string(COMMA));
            documents.push(line());
        }
        documents.push(node.build(context));
    }
    documents
}
