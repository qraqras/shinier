// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/lambda_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamLambdaNode {
    pub parameters: Option<Document>,
    pub body: Option<Document>,
}

pub fn layout_lambda_node(param: &LayoutParamLambdaNode) -> Document {
    Document::None
}
