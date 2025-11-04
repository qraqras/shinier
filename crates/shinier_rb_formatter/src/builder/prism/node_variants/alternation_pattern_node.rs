use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, line, space, string};
use crate::document::Document;
use crate::keyword::ALTERNATION;
use ruby_prism::AlternationPatternNode;
use ruby_prism::Node;

impl<'sh> Build for AlternationPatternNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        let flatten_node = flatten_alternation_pattern_node(self, context);
        let mut docs = Vec::new();
        for (i, part) in flatten_node.into_iter().enumerate() {
            if i > 0 {
                docs.push(space());
                docs.push(string(ALTERNATION));
                docs.push(line());
            }
            docs.push(part.build(context));
        }
        group(array(&docs))
    }
}

/// Recursively flattens nested alternation pattern nodes into a single vector of documents.
fn flatten_alternation_pattern_node<'sh>(
    node: &AlternationPatternNode<'sh>,
    context: &mut BuildContext,
) -> Vec<Node<'sh>> {
    let left = node.left();
    let right = node.right();
    let mut documents = Vec::new();
    // lhs
    match left.as_alternation_pattern_node() {
        Some(lhs) => {
            documents.extend(flatten_alternation_pattern_node(&lhs, context));
        }
        None => {
            documents.push(left);
        }
    }
    // rhs
    match right.as_alternation_pattern_node() {
        Some(rhs) => {
            documents.extend(flatten_alternation_pattern_node(&rhs, context));
        }
        None => {
            documents.push(right);
        }
    }
    documents
}
