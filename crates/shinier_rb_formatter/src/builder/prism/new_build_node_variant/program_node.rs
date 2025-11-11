// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/program_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::program_node::{layout_program_node, LayoutParamProgramNode};

fn build_program_node(node: &ProgramNode<'_>, context: &mut BuildContext) -> Document {
    let statements = build_node(&node.statements().as_node(), context);
    layout_program_node(&LayoutParamProgramNode { statements })
}
