// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/program_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamProgramNode {
    pub statements: Document,
}

pub fn layout_program_node(param: &LayoutParamProgramNode) -> Document {
    Document::None
}
