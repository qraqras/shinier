use crate::Document;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::program_node::LayoutParamProgramNode;
use crate::builder::prism::layout_node_variant::program_node::layout_program_node;
use ruby_prism::ProgramNode;

pub fn build_program_node(node: &ProgramNode<'_>, context: &mut BuildContext) -> Document {
    let statements = build_node(&node.statements().as_node(), context);
    layout_program_node(LayoutParamProgramNode { statements })
}
