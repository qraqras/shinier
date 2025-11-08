// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/parameters_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamParametersNode {
    pub requireds: Vec<Document>,
    pub optionals: Vec<Document>,
    pub rest: Option<Document>,
    pub posts: Vec<Document>,
    pub keywords: Vec<Document>,
    pub keyword: Option<Document>,
    pub block: Option<Document>,
}

pub fn layout_parameters_node(param: &LayoutParamParametersNode) -> Document {
    Document::None
}
