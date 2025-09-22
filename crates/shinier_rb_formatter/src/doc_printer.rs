use crate::ast::*;
use crate::doc::*;
use ruby_prism::*;

pub fn print_ast_to_doc(node: &Node) -> Doc {
    match node {
        Node::AliasGlobalVariableNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = AliasGlobalVariableNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
        }
        Node::AliasMethodNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = AliasMethodNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_alias_method_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::AlternationPatternNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = AlternationPatternNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_alternation_pattern_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::AndNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = AndNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_and_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ArgumentsNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ArgumentsNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_arguments_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ArrayNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ArrayNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_array_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ArrayPatternNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ArrayPatternNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_array_pattern_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::AssocNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = AssocNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_assoc_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::AssocSplatNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = AssocSplatNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_assoc_splat_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::BackReferenceReadNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = BackReferenceReadNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_back_reference_read_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::BeginNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = BeginNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_begin_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::BlockArgumentNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = BlockArgumentNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_block_argument_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::BlockLocalVariableNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = BlockLocalVariableNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_block_local_variable_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::BlockNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = BlockNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_block_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::BlockParameterNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = BlockParameterNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_block_parameter_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::BlockParametersNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = BlockParametersNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_block_parameters_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::BreakNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = BreakNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_break_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::CallAndWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = CallAndWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_call_and_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::CallNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = CallNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_call_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::CallOperatorWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = CallOperatorWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_call_operator_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::CallOrWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = CallOrWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_call_or_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::CallTargetNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = CallTargetNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_call_target_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::CapturePatternNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = CapturePatternNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_capture_pattern_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::CaseMatchNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = CaseMatchNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_case_match_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::CaseNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = CaseNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_case_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ClassNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ClassNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_class_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ClassVariableAndWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ClassVariableAndWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_class_variable_and_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ClassVariableOperatorWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ClassVariableOperatorWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_class_variable_operator_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ClassVariableOrWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ClassVariableOrWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_class_variable_or_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ClassVariableReadNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ClassVariableReadNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_class_variable_read_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::ClassVariableTargetNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ClassVariableTargetNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_class_variable_target_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::ClassVariableWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ClassVariableWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_class_variable_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ConstantAndWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ConstantAndWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_constant_and_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ConstantOperatorWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ConstantOperatorWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_constant_operator_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ConstantOrWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ConstantOrWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_constant_or_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ConstantPathAndWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ConstantPathAndWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_constant_path_and_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ConstantPathNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ConstantPathNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_constant_path_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ConstantPathOperatorWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ConstantPathOperatorWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_constant_path_operator_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ConstantPathOrWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ConstantPathOrWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_constant_path_or_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ConstantPathTargetNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ConstantPathTargetNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_constant_path_target_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ConstantPathWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ConstantPathWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_constant_path_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ConstantReadNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ConstantReadNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_constant_read_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::ConstantTargetNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ConstantTargetNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_constant_target_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::ConstantWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ConstantWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_constant_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::DefNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = DefNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_def_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::DefinedNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = DefinedNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_defined_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ElseNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ElseNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_else_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::EmbeddedStatementsNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = EmbeddedStatementsNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_embedded_statements_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::EmbeddedVariableNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = EmbeddedVariableNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_embedded_variable_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::EnsureNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = EnsureNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_ensure_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::FalseNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = FalseNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_false_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::FindPatternNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = FindPatternNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_find_pattern_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::FlipFlopNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = FlipFlopNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_flip_flop_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::FloatNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = FloatNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_float_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::ForNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ForNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_for_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ForwardingArgumentsNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ForwardingArgumentsNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_forwarding_arguments_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::ForwardingParameterNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ForwardingParameterNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_forwarding_parameter_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::ForwardingSuperNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ForwardingSuperNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_forwarding_super_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::GlobalVariableAndWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = GlobalVariableAndWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_global_variable_and_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::GlobalVariableOperatorWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = GlobalVariableOperatorWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_global_variable_operator_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::GlobalVariableOrWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = GlobalVariableOrWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_global_variable_or_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::GlobalVariableReadNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = GlobalVariableReadNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_global_variable_read_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::GlobalVariableTargetNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = GlobalVariableTargetNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_global_variable_target_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::GlobalVariableWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = GlobalVariableWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_global_variable_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::HashNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = HashNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_hash_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::HashPatternNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = HashPatternNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_hash_pattern_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::IfNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = IfNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_if_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ImaginaryNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ImaginaryNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_imaginary_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ImplicitNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ImplicitNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_implicit_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ImplicitRestNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ImplicitRestNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_implicit_rest_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::InNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = InNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_in_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::IndexAndWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = IndexAndWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_index_and_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::IndexOperatorWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = IndexOperatorWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_index_operator_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::IndexOrWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = IndexOrWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_index_or_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::IndexTargetNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = IndexTargetNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_index_target_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::InstanceVariableAndWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = InstanceVariableAndWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_instance_variable_and_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::InstanceVariableOperatorWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = InstanceVariableOperatorWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_instance_variable_operator_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::InstanceVariableOrWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = InstanceVariableOrWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_instance_variable_or_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::InstanceVariableReadNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = InstanceVariableReadNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_instance_variable_read_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::InstanceVariableTargetNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = InstanceVariableTargetNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_instance_variable_target_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::InstanceVariableWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = InstanceVariableWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_instance_variable_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::IntegerNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = IntegerNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_integer_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::InterpolatedMatchLastLineNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = InterpolatedMatchLastLineNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_interpolated_match_last_line_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::InterpolatedRegularExpressionNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = InterpolatedRegularExpressionNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_interpolated_regular_expression_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::InterpolatedStringNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = InterpolatedStringNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_interpolated_string_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::InterpolatedSymbolNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = InterpolatedSymbolNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_interpolated_symbol_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::InterpolatedXStringNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = InterpolatedXStringNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_interpolated_x_string_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ItLocalVariableReadNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ItLocalVariableReadNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_it_local_variable_read_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::ItParametersNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ItParametersNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_it_parameters_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::KeywordHashNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = KeywordHashNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_keyword_hash_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::KeywordRestParameterNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = KeywordRestParameterNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_keyword_rest_parameter_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::LambdaNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = LambdaNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_lambda_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::LocalVariableAndWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = LocalVariableAndWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_local_variable_and_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::LocalVariableOperatorWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = LocalVariableOperatorWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_local_variable_operator_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::LocalVariableOrWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = LocalVariableOrWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_local_variable_or_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::LocalVariableReadNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = LocalVariableReadNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_local_variable_read_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::LocalVariableTargetNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = LocalVariableTargetNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_local_variable_target_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::LocalVariableWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = LocalVariableWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_local_variable_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::MatchLastLineNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = MatchLastLineNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_match_last_line_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::MatchPredicateNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = MatchPredicateNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_match_predicate_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::MatchRequiredNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = MatchRequiredNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_match_required_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::MatchWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = MatchWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_match_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::MissingNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = MissingNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_missing_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::ModuleNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ModuleNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_module_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::MultiTargetNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = MultiTargetNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_multi_target_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::MultiWriteNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = MultiWriteNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_multi_write_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::NextNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = NextNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_next_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::NilNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = NilNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_nil_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::NoKeywordsParameterNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = NoKeywordsParameterNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_no_keywords_parameter_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::NumberedParametersNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = NumberedParametersNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_numbered_parameters_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::NumberedReferenceReadNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = NumberedReferenceReadNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_numbered_reference_read_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::OptionalKeywordParameterNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = OptionalKeywordParameterNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_optional_keyword_parameter_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::OptionalParameterNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = OptionalParameterNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_optional_parameter_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::OrNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = OrNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_or_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ParametersNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ParametersNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_parameters_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ParenthesesNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ParenthesesNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_parentheses_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::PinnedExpressionNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = PinnedExpressionNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_pinned_expression_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::PinnedVariableNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = PinnedVariableNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_pinned_variable_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::PostExecutionNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = PostExecutionNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_post_execution_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::PreExecutionNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = PreExecutionNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_pre_execution_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::ProgramNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ProgramNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_program_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::RangeNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = RangeNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_range_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::RationalNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = RationalNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_rational_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::RedoNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = RedoNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_redo_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::RegularExpressionNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = RegularExpressionNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_regular_expression_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::RequiredKeywordParameterNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = RequiredKeywordParameterNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_required_keyword_parameter_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::RequiredParameterNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = RequiredParameterNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_required_parameter_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::RescueModifierNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = RescueModifierNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_rescue_modifier_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::RescueNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = RescueNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_rescue_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::RestParameterNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = RestParameterNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_rest_parameter_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::RetryNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = RetryNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_retry_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::ReturnNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ReturnNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_return_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::SelfNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = SelfNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_self_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::ShareableConstantNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = ShareableConstantNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_shareable_constant_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::SingletonClassNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = SingletonClassNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_singleton_class_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::SourceEncodingNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = SourceEncodingNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_source_encoding_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::SourceFileNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = SourceFileNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_source_file_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::SourceLineNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = SourceLineNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_source_line_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::SplatNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = SplatNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_splat_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::StatementsNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = StatementsNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_statements_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::StringNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = StringNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_string_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::SuperNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = SuperNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_super_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::SymbolNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = SymbolNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_symbol_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::TrueNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = TrueNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_true_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::UndefNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = UndefNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_undef_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::UnlessNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = UnlessNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_unless_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::UntilNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = UntilNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_until_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::WhenNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = WhenNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_when_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::WhileNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = WhileNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_while_node(&concrete);
            self.visit_branch_node_leave();
        }
        Node::XStringNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = XStringNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_leaf_node_enter(concrete.as_node());
            self.visit_x_string_node(&concrete);
            self.visit_leaf_node_leave();
        }
        Node::YieldNode {
            parser,
            pointer,
            marker,
        } => {
            let concrete = YieldNode {
                parser: *parser,
                pointer: *pointer,
                marker: *marker,
            };
            self.visit_branch_node_enter(concrete.as_node());
            self.visit_yield_node(&concrete);
            self.visit_branch_node_leave();
        }
    }
}
