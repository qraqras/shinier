use crate::buildable::Buildable;
use crate::builder::prism_node::node::*;
use crate::document::Document;
use ruby_prism::Node;

impl<'a> Buildable<'_> for Node<'_> {
    #[rustfmt::skip]
    fn build(&self) -> Document {
        match self {
            Node::AliasGlobalVariableNode { .. } => {
                println_dbg!("AliasGlobalVariableNode");
                alias_global_variable_node::build_node(self.as_alias_global_variable_node().as_ref())
            }
            Node::AliasMethodNode { .. } => {
                println_dbg!("AliasMethodNode");
                alias_method_node::build_node(self.as_alias_method_node().as_ref())
            }
            Node::AlternationPatternNode { .. } => {
                println_dbg!("AlternationPatternNode");
                alternation_pattern_node::build_node(self.as_alternation_pattern_node().as_ref())
            }
            Node::AndNode { .. } => {
                println_dbg!("AndNode");
                and_node::build_node(self.as_and_node().as_ref())
            }
            Node::ArgumentsNode { .. } => {
                println_dbg!("ArgumentsNode");
                arguments_node::build_node(self.as_arguments_node().as_ref())
            }
            Node::ArrayNode { .. } => {
                println_dbg!("ArrayNode");
                array_node::build_node(self.as_array_node().as_ref())
            }
            Node::ArrayPatternNode { .. } => {
                println_dbg!("ArrayPatternNode");
                array_pattern_node::build_node(self.as_array_pattern_node().as_ref())
            }
            Node::AssocNode { .. } => {
                println_dbg!("AssocNode");
                assoc_node::build_node(self.as_assoc_node().as_ref())
            }
            Node::AssocSplatNode { .. } => {
                println_dbg!("AssocSplatNode");
                assoc_splat_node::build_node(self.as_assoc_splat_node().as_ref())
            }
            Node::BackReferenceReadNode { .. } => {
                println_dbg!("BackReferenceReadNode");
                back_reference_read_node::build_node(self.as_back_reference_read_node().as_ref())
            }
            Node::BeginNode { .. } => {
                println_dbg!("BeginNode");
                begin_node::build_node(self.as_begin_node().as_ref())
            }
            Node::BlockArgumentNode { .. } => {
                println_dbg!("BlockArgumentNode");
                block_argument_node::build_node(self.as_block_argument_node().as_ref())
            }
            Node::BlockLocalVariableNode { .. } => {
                println_dbg!("BlockLocalVariableNode");
                block_local_variable_node::build_node(self.as_block_local_variable_node().as_ref())
            }
            Node::BlockNode { .. } => {
                println_dbg!("BlockNode");
                block_node::build_node(self.as_block_node().as_ref())
            }
            Node::BlockParameterNode { .. } => {
                println_dbg!("BlockParameterNode");
                block_parameter_node::build_node(self.as_block_parameter_node().as_ref())
            }
            Node::BlockParametersNode { .. } => {
                println_dbg!("BlockParametersNode");
                block_parameters_node::build_node(self.as_block_parameters_node().as_ref())
            }
            Node::BreakNode { .. } => {
                println_dbg!("BreakNode");
                break_node::build_node(self.as_break_node().as_ref())
            }
            Node::CallAndWriteNode { .. } => {
                println_dbg!("CallAndWriteNode");
                call_and_write_node::build_node(self.as_call_and_write_node().as_ref())
            }
            Node::CallNode { .. } => {
                println_dbg!("CallNode");
                call_node::build_node(self.as_call_node().as_ref())
            }
            Node::CallOperatorWriteNode { .. } => {
                println_dbg!("CallOperatorWriteNode");
                call_operator_write_node::build_node(self.as_call_operator_write_node().as_ref())
            }
            Node::CallOrWriteNode { .. } => {
                println_dbg!("CallOrWriteNode");
                call_or_write_node::build_node(self.as_call_or_write_node().as_ref())
            }
            Node::CallTargetNode { .. } => {
                println_dbg!("CallTargetNode");
                call_target_node::build_node(self.as_call_target_node().as_ref())
            }
            Node::CapturePatternNode { .. } => {
                println_dbg!("CapturePatternNode");
                capture_pattern_node::build_node(self.as_capture_pattern_node().as_ref())
            }
            Node::CaseMatchNode { .. } => {
                println_dbg!("CaseMatchNode");
                case_match_node::build_node(self.as_case_match_node().as_ref())
            }
            Node::CaseNode { .. } => {
                println_dbg!("CaseNode");
                case_node::build_node(self.as_case_node().as_ref())
            }
            Node::ClassNode { .. } => {
                println_dbg!("ClassNode");
                class_node::build_node(self.as_class_node().as_ref())
            }
            Node::ClassVariableAndWriteNode { .. } => {
                println_dbg!("ClassVariableAndWriteNode");
                class_variable_and_write_node::build_node(self.as_class_variable_and_write_node().as_ref())
            }
            Node::ClassVariableOperatorWriteNode { .. } => {
                println_dbg!("ClassVariableOperatorWriteNode");
                class_variable_operator_write_node::build_node(self.as_class_variable_operator_write_node().as_ref())
            }
            Node::ClassVariableOrWriteNode { .. } => {
                println_dbg!("ClassVariableOrWriteNode");
                class_variable_or_write_node::build_node(self.as_class_variable_or_write_node().as_ref())
            }
            Node::ClassVariableReadNode { .. } => {
                println_dbg!("ClassVariableReadNode");
                class_variable_read_node::build_node(self.as_class_variable_read_node().as_ref())
            }
            Node::ClassVariableTargetNode { .. } => {
                println_dbg!("ClassVariableTargetNode");
                class_variable_target_node::build_node(self.as_class_variable_target_node().as_ref())
            }
            Node::ClassVariableWriteNode { .. } => {
                println_dbg!("ClassVariableWriteNode");
                class_variable_write_node::build_node(self.as_class_variable_write_node().as_ref())
            }
            Node::ConstantAndWriteNode { .. } => {
                println_dbg!("ConstantAndWriteNode");
                constant_and_write_node::build_node(self.as_constant_and_write_node().as_ref())
            }
            Node::ConstantOperatorWriteNode { .. } => {
                println_dbg!("ConstantOperatorWriteNode");
                constant_operator_write_node::build_node(self.as_constant_operator_write_node().as_ref())
            }
            Node::ConstantOrWriteNode { .. } => {
                println_dbg!("ConstantOrWriteNode");
                constant_or_write_node::build_node(self.as_constant_or_write_node().as_ref())
            }
            Node::ConstantPathAndWriteNode { .. } => {
                println_dbg!("ConstantPathAndWriteNode");
                constant_path_and_write_node::build_node(self.as_constant_path_and_write_node().as_ref())
            }
            Node::ConstantPathNode { .. } => {
                println_dbg!("ConstantPathNode");
                constant_path_node::build_node(self.as_constant_path_node().as_ref())
            }
            Node::ConstantPathOperatorWriteNode { .. } => {
                println_dbg!("ConstantPathOperatorWriteNode");
                constant_path_operator_write_node::build_node(self.as_constant_path_operator_write_node().as_ref())
            }
            Node::ConstantPathOrWriteNode { .. } => {
                println_dbg!("ConstantPathOrWriteNode");
                constant_path_or_write_node::build_node(self.as_constant_path_or_write_node().as_ref())
            }
            Node::ConstantPathTargetNode { .. } => {
                println_dbg!("ConstantPathTargetNode");
                constant_path_target_node::build_node(self.as_constant_path_target_node().as_ref())
            }
            Node::ConstantPathWriteNode { .. } => {
                println_dbg!("ConstantPathWriteNode");
                constant_path_write_node::build_node(self.as_constant_path_write_node().as_ref())
            }
            Node::ConstantReadNode { .. } => {
                println_dbg!("ConstantReadNode");
                constant_read_node::build_node(self.as_constant_read_node().as_ref())
            }
            Node::ConstantTargetNode { .. } => {
                println_dbg!("ConstantTargetNode");
                constant_target_node::build_node(self.as_constant_target_node().as_ref())
            }
            Node::ConstantWriteNode { .. } => {
                println_dbg!("ConstantWriteNode");
                constant_write_node::build_node(self.as_constant_write_node().as_ref())
            }
            Node::DefNode { .. } => {
                println_dbg!("DefNode");
                def_node::build_node(self.as_def_node().as_ref())
            }
            Node::DefinedNode { .. } => {
                println_dbg!("DefinedNode");
                defined_node::build_node(self.as_defined_node().as_ref())
            }
            Node::ElseNode { .. } => {
                println_dbg!("ElseNode");
                else_node::build_node(self.as_else_node().as_ref())
            }
            Node::EmbeddedStatementsNode { .. } => {
                println_dbg!("EmbeddedStatementsNode");
                embedded_statements_node::build_node(self.as_embedded_statements_node().as_ref())
            }
            Node::EmbeddedVariableNode { .. } => {
                println_dbg!("EmbeddedVariableNode");
                embedded_variable_node::build_node(self.as_embedded_variable_node().as_ref())
            }
            Node::EnsureNode { .. } => {
                println_dbg!("EnsureNode");
                ensure_node::build_node(self.as_ensure_node().as_ref())
            }
            Node::FalseNode { .. } => {
                println_dbg!("FalseNode");
                false_node::build_node(self.as_false_node().as_ref())
            }
            Node::FindPatternNode { .. } => {
                println_dbg!("FindPatternNode");
                find_pattern_node::build_node(self.as_find_pattern_node().as_ref())
            }
            Node::FlipFlopNode { .. } => {
                println_dbg!("FlipFlopNode");
                flip_flop_node::build_node(self.as_flip_flop_node().as_ref())
            }
            Node::FloatNode { .. } => {
                println_dbg!("FloatNode");
                float_node::build_node(self.as_float_node().as_ref())
            }
            Node::ForNode { .. } => {
                println_dbg!("ForNode");
                for_node::build_node(self.as_for_node().as_ref())
            }
            Node::ForwardingArgumentsNode { .. } => {
                println_dbg!("ForwardingArgumentsNode");
                forwarding_arguments_node::build_node(self.as_forwarding_arguments_node().as_ref())
            }
            Node::ForwardingParameterNode { .. } => {
                println_dbg!("ForwardingParameterNode");
                forwarding_parameter_node::build_node(self.as_forwarding_parameter_node().as_ref())
            }
            Node::ForwardingSuperNode { .. } => {
                println_dbg!("ForwardingSuperNode");
                forwarding_super_node::build_node(self.as_forwarding_super_node().as_ref())
            }
            Node::GlobalVariableAndWriteNode { .. } => {
                println_dbg!("GlobalVariableAndWriteNode");
                global_variable_and_write_node::build_node(self.as_global_variable_and_write_node().as_ref())
            }
            Node::GlobalVariableOperatorWriteNode { .. } => {
                println_dbg!("GlobalVariableOperatorWriteNode");
                global_variable_operator_write_node::build_node(self.as_global_variable_operator_write_node().as_ref())
            }
            Node::GlobalVariableOrWriteNode { .. } => {
                println_dbg!("GlobalVariableOrWriteNode");
                global_variable_or_write_node::build_node(self.as_global_variable_or_write_node().as_ref())
            }
            Node::GlobalVariableReadNode { .. } => {
                println_dbg!("GlobalVariableReadNode");
                global_variable_read_node::build_node(self.as_global_variable_read_node().as_ref())
            }
            Node::GlobalVariableTargetNode { .. } => {
                println_dbg!("GlobalVariableTargetNode");
                global_variable_target_node::build_node(self.as_global_variable_target_node().as_ref())
            }
            Node::GlobalVariableWriteNode { .. } => {
                println_dbg!("GlobalVariableWriteNode");
                global_variable_write_node::build_node(self.as_global_variable_write_node().as_ref())
            }
            Node::HashNode { .. } => {
                println_dbg!("HashNode");
                hash_node::build_node(self.as_hash_node().as_ref())
            }
            Node::HashPatternNode { .. } => {
                println_dbg!("HashPatternNode");
                hash_pattern_node::build_node(self.as_hash_pattern_node().as_ref())
            }
            Node::IfNode { .. } => {
                println_dbg!("IfNode");
                if_node::build_node(self.as_if_node().as_ref())
            }
            Node::ImaginaryNode { .. } => {
                println_dbg!("ImaginaryNode");
                imaginary_node::build_node(self.as_imaginary_node().as_ref())
            }
            Node::ImplicitNode { .. } => {
                println_dbg!("ImplicitNode");
                implicit_node::build_node(self.as_implicit_node().as_ref())
            }
            Node::ImplicitRestNode { .. } => {
                println_dbg!("ImplicitRestNode");
                implicit_rest_node::build_node(self.as_implicit_rest_node().as_ref())
            }
            Node::InNode { .. } => {
                println_dbg!("InNode");
                in_node::build_node(self.as_in_node().as_ref())
            }
            Node::IndexAndWriteNode { .. } => {
                println_dbg!("IndexAndWriteNode");
                index_and_write_node::build_node(self.as_index_and_write_node().as_ref())
            }
            Node::IndexOperatorWriteNode { .. } => {
                println_dbg!("IndexOperatorWriteNode");
                index_operator_write_node::build_node(self.as_index_operator_write_node().as_ref())
            }
            Node::IndexOrWriteNode { .. } => {
                println_dbg!("IndexOrWriteNode");
                index_or_write_node::build_node(self.as_index_or_write_node().as_ref())
            }
            Node::IndexTargetNode { .. } => {
                println_dbg!("IndexTargetNode");
                index_target_node::build_node(self.as_index_target_node().as_ref())
            }
            Node::InstanceVariableAndWriteNode { .. } => {
                println_dbg!("InstanceVariableAndWriteNode");
                instance_variable_and_write_node::build_node(self.as_instance_variable_and_write_node().as_ref())
            }
            Node::InstanceVariableOperatorWriteNode { .. } => {
                println_dbg!("InstanceVariableOperatorWriteNode");
                instance_variable_operator_write_node::build_node(self.as_instance_variable_operator_write_node().as_ref())
            }
            Node::InstanceVariableOrWriteNode { .. } => {
                println_dbg!("InstanceVariableOrWriteNode");
                instance_variable_or_write_node::build_node(self.as_instance_variable_or_write_node().as_ref())
            }
            Node::InstanceVariableReadNode { .. } => {
                println_dbg!("InstanceVariableReadNode");
                instance_variable_read_node::build_node(self.as_instance_variable_read_node().as_ref())
            }
            Node::InstanceVariableTargetNode { .. } => {
                println_dbg!("InstanceVariableTargetNode");
                instance_variable_target_node::build_node(self.as_instance_variable_target_node().as_ref())
            }
            Node::InstanceVariableWriteNode { .. } => {
                println_dbg!("InstanceVariableWriteNode");
                instance_variable_write_node::build_node(self.as_instance_variable_write_node().as_ref())
            }
            Node::IntegerNode { .. } => {
                println_dbg!("IntegerNode");
                integer_node::build_node(self.as_integer_node().as_ref())
            }
            Node::InterpolatedMatchLastLineNode { .. } => {
                println_dbg!("InterpolatedMatchLastLineNode");
                interpolated_match_last_line_node::build_node(self.as_interpolated_match_last_line_node().as_ref())
            }
            Node::InterpolatedRegularExpressionNode { .. } => {
                println_dbg!("InterpolatedRegularExpressionNode");
                interpolated_regular_expression_node::build_node(self.as_interpolated_regular_expression_node().as_ref())
            }
            Node::InterpolatedStringNode { .. } => {
                println_dbg!("InterpolatedStringNode");
                interpolated_string_node::build_node(self.as_interpolated_string_node().as_ref())
            }
            Node::InterpolatedSymbolNode { .. } => {
                println_dbg!("InterpolatedSymbolNode");
                interpolated_symbol_node::build_node(self.as_interpolated_symbol_node().as_ref())
            }
            Node::InterpolatedXStringNode { .. } => {
                println_dbg!("InterpolatedXStringNode");
                interpolated_x_string_node::build_node(self.as_interpolated_x_string_node().as_ref())
            }
            Node::ItLocalVariableReadNode { .. } => {
                println_dbg!("ItLocalVariableReadNode");
                it_local_variable_read_node::build_node(self.as_it_local_variable_read_node().as_ref())
            }
            Node::ItParametersNode { .. } => {
                println_dbg!("ItParametersNode");
                it_parameters_node::build_node(self.as_it_parameters_node().as_ref())
            }
            Node::KeywordHashNode { .. } => {
                println_dbg!("KeywordHashNode");
                keyword_hash_node::build_node(self.as_keyword_hash_node().as_ref())
            }
            Node::KeywordRestParameterNode { .. } => {
                println_dbg!("KeywordRestParameterNode");
                keyword_rest_parameter_node::build_node(self.as_keyword_rest_parameter_node().as_ref())
            }
            Node::LambdaNode { .. } => {
                println_dbg!("LambdaNode");
                lambda_node::build_node(self.as_lambda_node().as_ref())
            }
            Node::LocalVariableAndWriteNode { .. } => {
                println_dbg!("LocalVariableAndWriteNode");
                local_variable_and_write_node::build_node(self.as_local_variable_and_write_node().as_ref())
            }
            Node::LocalVariableOperatorWriteNode { .. } => {
                println_dbg!("LocalVariableOperatorWriteNode");
                local_variable_operator_write_node::build_node(self.as_local_variable_operator_write_node().as_ref())
            }
            Node::LocalVariableOrWriteNode { .. } => {
                println_dbg!("LocalVariableOrWriteNode");
                local_variable_or_write_node::build_node(self.as_local_variable_or_write_node().as_ref())
            }
            Node::LocalVariableReadNode { .. } => {
                println_dbg!("LocalVariableReadNode");
                local_variable_read_node::build_node(self.as_local_variable_read_node().as_ref())
            }
            Node::LocalVariableTargetNode { .. } => {
                println_dbg!("LocalVariableTargetNode");
                local_variable_target_node::build_node(self.as_local_variable_target_node().as_ref())
            }
            Node::LocalVariableWriteNode { .. } => {
                println_dbg!("LocalVariableWriteNode");
                local_variable_write_node::build_node(self.as_local_variable_write_node().as_ref())
            }
            Node::MatchLastLineNode { .. } => {
                println_dbg!("MatchLastLineNode");
                match_last_line_node::build_node(self.as_match_last_line_node().as_ref())
            }
            Node::MatchPredicateNode { .. } => {
                println_dbg!("MatchPredicateNode");
                match_predicate_node::build_node(self.as_match_predicate_node().as_ref())
            }
            Node::MatchRequiredNode { .. } => {
                println_dbg!("MatchRequiredNode");
                match_required_node::build_node(self.as_match_required_node().as_ref())
            }
            Node::MatchWriteNode { .. } => {
                println_dbg!("MatchWriteNode");
                match_write_node::build_node(self.as_match_write_node().as_ref())
            }
            Node::MissingNode { .. } => {
                println_dbg!("MissingNode");
                missing_node::build_node(self.as_missing_node().as_ref())
            }
            Node::ModuleNode { .. } => {
                println_dbg!("ModuleNode");
                module_node::build_node(self.as_module_node().as_ref())
            }
            Node::MultiTargetNode { .. } => {
                println_dbg!("MultiTargetNode");
                multi_target_node::build_node(self.as_multi_target_node().as_ref())
            }
            Node::MultiWriteNode { .. } => {
                println_dbg!("MultiWriteNode");
                multi_write_node::build_node(self.as_multi_write_node().as_ref())
            }
            Node::NextNode { .. } => {
                println_dbg!("NextNode");
                next_node::build_node(self.as_next_node().as_ref())
            }
            Node::NilNode { .. } => {
                println_dbg!("NilNode");
                nil_node::build_node(self.as_nil_node().as_ref())
            }
            Node::NoKeywordsParameterNode { .. } => {
                println_dbg!("NoKeywordsParameterNode");
                no_keywords_parameter_node::build_node(self.as_no_keywords_parameter_node().as_ref())
            }
            Node::NumberedParametersNode { .. } => {
                println_dbg!("NumberedParametersNode");
                numbered_parameters_node::build_node(self.as_numbered_parameters_node().as_ref())
            }
            Node::NumberedReferenceReadNode { .. } => {
                println_dbg!("NumberedReferenceReadNode");
                numbered_reference_read_node::build_node(self.as_numbered_reference_read_node().as_ref())
            }
            Node::OptionalKeywordParameterNode { .. } => {
                println_dbg!("OptionalKeywordParameterNode");
                optional_keyword_parameter_node::build_node(self.as_optional_keyword_parameter_node().as_ref())
            }
            Node::OptionalParameterNode { .. } => {
                println_dbg!("OptionalParameterNode");
                optional_parameter_node::build_node(self.as_optional_parameter_node().as_ref())
            }
            Node::OrNode { .. } => {
                println_dbg!("OrNode");
                or_node::build_node(self.as_or_node().as_ref())
            }
            Node::ParametersNode { .. } => {
                println_dbg!("ParametersNode");
                parameters_node::build_node(self.as_parameters_node().as_ref())
            }
            Node::ParenthesesNode { .. } => {
                println_dbg!("ParenthesesNode");
                parentheses_node::build_node(self.as_parentheses_node().as_ref())
            }
            Node::PinnedExpressionNode { .. } => {
                println_dbg!("PinnedExpressionNode");
                pinned_expression_node::build_node(self.as_pinned_expression_node().as_ref())
            }
            Node::PinnedVariableNode { .. } => {
                println_dbg!("PinnedVariableNode");
                pinned_variable_node::build_node(self.as_pinned_variable_node().as_ref())
            }
            Node::PostExecutionNode { .. } => {
                println_dbg!("PostExecutionNode");
                post_execution_node::build_node(self.as_post_execution_node().as_ref())
            }
            Node::PreExecutionNode { .. } => {
                println_dbg!("PreExecutionNode");
                pre_execution_node::build_node(self.as_pre_execution_node().as_ref())
            }
            Node::ProgramNode { .. } => {
                println_dbg!("ProgramNode");
                program_node::build_node(self.as_program_node().as_ref())
            }
            Node::RangeNode { .. } => {
                println_dbg!("RangeNode");
                range_node::build_node(self.as_range_node().as_ref())
            }
            Node::RationalNode { .. } => {
                println_dbg!("RationalNode");
                rational_node::build_node(self.as_rational_node().as_ref())
            }
            Node::RedoNode { .. } => {
                println_dbg!("RedoNode");
                redo_node::build_node(self.as_redo_node().as_ref())
            }
            Node::RegularExpressionNode { .. } => {
                println_dbg!("RegularExpressionNode");
                regular_expression_node::build_node(self.as_regular_expression_node().as_ref())
            }
            Node::RequiredKeywordParameterNode { .. } => {
                println_dbg!("RequiredKeywordParameterNode");
                required_keyword_parameter_node::build_node(self.as_required_keyword_parameter_node().as_ref())
            }
            Node::RequiredParameterNode { .. } => {
                println_dbg!("RequiredParameterNode");
                required_parameter_node::build_node(self.as_required_parameter_node().as_ref())
            }
            Node::RescueModifierNode { .. } => {
                println_dbg!("RescueModifierNode");
                rescue_modifier_node::build_node(self.as_rescue_modifier_node().as_ref())
            }
            Node::RescueNode { .. } => {
                println_dbg!("RescueNode");
                rescue_node::build_node(self.as_rescue_node().as_ref())
            }
            Node::RestParameterNode { .. } => {
                println_dbg!("RestParameterNode");
                rest_parameter_node::build_node(self.as_rest_parameter_node().as_ref())
            }
            Node::RetryNode { .. } => {
                println_dbg!("RetryNode");
                retry_node::build_node(self.as_retry_node().as_ref())
            }
            Node::ReturnNode { .. } => {
                println_dbg!("ReturnNode");
                return_node::build_node(self.as_return_node().as_ref())
            }
            Node::SelfNode { .. } => {
                println_dbg!("SelfNode");
                self_node::build_node(self.as_self_node().as_ref())
            }
            Node::ShareableConstantNode { .. } => {
                println_dbg!("ShareableConstantNode");
                shareable_constant_node::build_node(self.as_shareable_constant_node().as_ref())
            }
            Node::SingletonClassNode { .. } => {
                println_dbg!("SingletonClassNode");
                singleton_class_node::build_node(self.as_singleton_class_node().as_ref())
            }
            Node::SourceEncodingNode { .. } => {
                println_dbg!("SourceEncodingNode");
                source_encoding_node::build_node(self.as_source_encoding_node().as_ref())
            }
            Node::SourceFileNode { .. } => {
                println_dbg!("SourceFileNode");
                source_file_node::build_node(self.as_source_file_node().as_ref())
            }
            Node::SourceLineNode { .. } => {
                println_dbg!("SourceLineNode");
                source_line_node::build_node(self.as_source_line_node().as_ref())
            }
            Node::SplatNode { .. } => {
                println_dbg!("SplatNode");
                splat_node::build_node(self.as_splat_node().as_ref())
            }
            Node::StatementsNode { .. } => {
                println_dbg!("StatementsNode");
                statements_node::build_node(self.as_statements_node().as_ref())
            }
            Node::StringNode { .. } => {
                println_dbg!("StringNode");
                string_node::build_node(self.as_string_node().as_ref())
            }
            Node::SuperNode { .. } => {
                println_dbg!("SuperNode");
                super_node::build_node(self.as_super_node().as_ref())
            }
            Node::SymbolNode { .. } => {
                println_dbg!("SymbolNode");
                symbol_node::build_node(self.as_symbol_node().as_ref())
            }
            Node::TrueNode { .. } => {
                println_dbg!("TrueNode");
                true_node::build_node(self.as_true_node().as_ref())
            }
            Node::UndefNode { .. } => {
                println_dbg!("UndefNode");
                undef_node::build_node(self.as_undef_node().as_ref())
            }
            Node::UnlessNode { .. } => {
                println_dbg!("UnlessNode");
                unless_node::build_node(self.as_unless_node().as_ref())
            }
            Node::UntilNode { .. } => {
                println_dbg!("UntilNode");
                until_node::build_node(self.as_until_node().as_ref())
            }
            Node::WhenNode { .. } => {
                println_dbg!("WhenNode");
                when_node::build_node(self.as_when_node().as_ref())
            }
            Node::WhileNode { .. } => {
                println_dbg!("WhileNode");
                while_node::build_node(self.as_while_node().as_ref())
            }
            Node::XStringNode { .. } => {
                println_dbg!("XStringNode");
                x_string_node::build_node(self.as_x_string_node().as_ref())
            }
            Node::YieldNode { .. } => {
                println_dbg!("YieldNode");
                yield_node::build_node(self.as_yield_node().as_ref())
            }
        }
    }
}
