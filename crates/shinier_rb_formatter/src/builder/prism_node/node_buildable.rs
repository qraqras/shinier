use crate::buildable::Buildable;
use crate::builder::prism_node::node::*;
use crate::document::Document;
use ruby_prism::Node;

impl<'a> Buildable<'_> for Node<'_> {
    #[rustfmt::skip]
    fn build(&self) -> Document {
        match self {
            Node::AliasGlobalVariableNode { .. } => {
                alias_global_variable_node::build_node(self.as_alias_global_variable_node().as_ref())
            }
            Node::AliasMethodNode { .. } => {
                alias_method_node::build_node(self.as_alias_method_node().as_ref())
            }
            Node::AlternationPatternNode { .. } => {
                alternation_pattern_node::build_node(self.as_alternation_pattern_node().as_ref())
            }
            Node::AndNode { .. } => {
                and_node::build_node(self.as_and_node().as_ref())
            }
            Node::ArgumentsNode { .. } => {
                arguments_node::build_node(self.as_arguments_node().as_ref())
            }
            Node::ArrayNode { .. } => {
                array_node::build_node(self.as_array_node().as_ref())
            }
            Node::ArrayPatternNode { .. } => {
                array_pattern_node::build_node(self.as_array_pattern_node().as_ref())
            }
            Node::AssocNode { .. } => {
                assoc_node::build_node(self.as_assoc_node().as_ref())
            }
            Node::AssocSplatNode { .. } => {
                assoc_splat_node::build_node(self.as_assoc_splat_node().as_ref())
            }
            Node::BackReferenceReadNode { .. } => {
                back_reference_read_node::build_node(self.as_back_reference_read_node().as_ref())
            }
            Node::BeginNode { .. } => {
                begin_node::build_node(self.as_begin_node().as_ref())
            }
            Node::BlockArgumentNode { .. } => {
                block_argument_node::build_node(self.as_block_argument_node().as_ref())
            }
            Node::BlockLocalVariableNode { .. } => {
                block_local_variable_node::build_node(self.as_block_local_variable_node().as_ref())
            }
            Node::BlockNode { .. } => {
                block_node::build_node(self.as_block_node().as_ref())
            }
            Node::BlockParameterNode { .. } => {
                block_parameter_node::build_node(self.as_block_parameter_node().as_ref())
            }
            Node::BlockParametersNode { .. } => {
                block_parameters_node::build_node(self.as_block_parameters_node().as_ref())
            }
            Node::BreakNode { .. } => {
                break_node::build_node(self.as_break_node().as_ref())
            }
            Node::CallAndWriteNode { .. } => {
                call_and_write_node::build_node(self.as_call_and_write_node().as_ref())
            }
            Node::CallNode { .. } => {
                call_node::build_node(self.as_call_node().as_ref())
            }
            Node::CallOperatorWriteNode { .. } => {
                call_operator_write_node::build_node(self.as_call_operator_write_node().as_ref())
            }
            Node::CallOrWriteNode { .. } => {
                call_or_write_node::build_node(self.as_call_or_write_node().as_ref())
            }
            Node::CallTargetNode { .. } => {
                call_target_node::build_node(self.as_call_target_node().as_ref())
            }
            Node::CapturePatternNode { .. } => {
                capture_pattern_node::build_node(self.as_capture_pattern_node().as_ref())
            }
            Node::CaseMatchNode { .. } => {
                case_match_node::build_node(self.as_case_match_node().as_ref())
            }
            Node::CaseNode { .. } => {
                case_node::build_node(self.as_case_node().as_ref())
            }
            Node::ClassNode { .. } => {
                class_node::build_node(self.as_class_node().as_ref())
            }
            Node::ClassVariableAndWriteNode { .. } => {
                class_variable_and_write_node::build_node(self.as_class_variable_and_write_node().as_ref())
            }
            Node::ClassVariableOperatorWriteNode { .. } => {
                class_variable_operator_write_node::build_node(self.as_class_variable_operator_write_node().as_ref())
            }
            Node::ClassVariableOrWriteNode { .. } => {
                class_variable_or_write_node::build_node(self.as_class_variable_or_write_node().as_ref())
            }
            Node::ClassVariableReadNode { .. } => {
                class_variable_read_node::build_node(self.as_class_variable_read_node().as_ref())
            }
            Node::ClassVariableTargetNode { .. } => {
                class_variable_target_node::build_node(self.as_class_variable_target_node().as_ref())
            }
            Node::ClassVariableWriteNode { .. } => {
                class_variable_write_node::build_node(self.as_class_variable_write_node().as_ref())
            }
            Node::ConstantAndWriteNode { .. } => {
                constant_and_write_node::build_node(self.as_constant_and_write_node().as_ref())
            }
            Node::ConstantOperatorWriteNode { .. } => {
                constant_operator_write_node::build_node(self.as_constant_operator_write_node().as_ref())
            }
            Node::ConstantOrWriteNode { .. } => {
                constant_or_write_node::build_node(self.as_constant_or_write_node().as_ref())
            }
            Node::ConstantPathAndWriteNode { .. } => {
                constant_path_and_write_node::build_node(self.as_constant_path_and_write_node().as_ref())
            }
            Node::ConstantPathNode { .. } => {
                constant_path_node::build_node(self.as_constant_path_node().as_ref())
            }
            Node::ConstantPathOperatorWriteNode { .. } => {
                constant_path_operator_write_node::build_node(self.as_constant_path_operator_write_node().as_ref())
            }
            Node::ConstantPathOrWriteNode { .. } => {
                constant_path_or_write_node::build_node(self.as_constant_path_or_write_node().as_ref())
            }
            Node::ConstantPathTargetNode { .. } => {
                constant_path_target_node::build_node(self.as_constant_path_target_node().as_ref())
            }
            Node::ConstantPathWriteNode { .. } => {
                constant_path_write_node::build_node(self.as_constant_path_write_node().as_ref())
            }
            Node::ConstantReadNode { .. } => {
                constant_read_node::build_node(self.as_constant_read_node().as_ref())
            }
            Node::ConstantTargetNode { .. } => {
                constant_target_node::build_node(self.as_constant_target_node().as_ref())
            }
            Node::ConstantWriteNode { .. } => {
                constant_write_node::build_node(self.as_constant_write_node().as_ref())
            }
            Node::DefNode { .. } => {
                def_node::build_node(self.as_def_node().as_ref())
            }
            Node::DefinedNode { .. } => {
                defined_node::build_node(self.as_defined_node().as_ref())
            }
            Node::ElseNode { .. } => {
                else_node::build_node(self.as_else_node().as_ref())
            }
            Node::EmbeddedStatementsNode { .. } => {
                embedded_statements_node::build_node(self.as_embedded_statements_node().as_ref())
            }
            Node::EmbeddedVariableNode { .. } => {
                embedded_variable_node::build_node(self.as_embedded_variable_node().as_ref())
            }
            Node::EnsureNode { .. } => {
                ensure_node::build_node(self.as_ensure_node().as_ref())
            }
            Node::FalseNode { .. } => {
                false_node::build_node(self.as_false_node().as_ref())
            }
            Node::FindPatternNode { .. } => {
                find_pattern_node::build_node(self.as_find_pattern_node().as_ref())
            }
            Node::FlipFlopNode { .. } => {
                flip_flop_node::build_node(self.as_flip_flop_node().as_ref())
            }
            Node::FloatNode { .. } => {
                float_node::build_node(self.as_float_node().as_ref())
            }
            Node::ForNode { .. } => {
                for_node::build_node(self.as_for_node().as_ref())
            }
            Node::ForwardingArgumentsNode { .. } => {
                forwarding_arguments_node::build_node(self.as_forwarding_arguments_node().as_ref())
            }
            Node::ForwardingParameterNode { .. } => {
                forwarding_parameter_node::build_node(self.as_forwarding_parameter_node().as_ref())
            }
            Node::ForwardingSuperNode { .. } => {
                forwarding_super_node::build_node(self.as_forwarding_super_node().as_ref())
            }
            Node::GlobalVariableAndWriteNode { .. } => {
                global_variable_and_write_node::build_node(self.as_global_variable_and_write_node().as_ref())
            }
            Node::GlobalVariableOperatorWriteNode { .. } => {
                global_variable_operator_write_node::build_node(self.as_global_variable_operator_write_node().as_ref())
            }
            Node::GlobalVariableOrWriteNode { .. } => {
                global_variable_or_write_node::build_node(self.as_global_variable_or_write_node().as_ref())
            }
            Node::GlobalVariableReadNode { .. } => {
                global_variable_read_node::build_node(self.as_global_variable_read_node().as_ref())
            }
            Node::GlobalVariableTargetNode { .. } => {
                global_variable_target_node::build_node(self.as_global_variable_target_node().as_ref())
            }
            Node::GlobalVariableWriteNode { .. } => {
                global_variable_write_node::build_node(self.as_global_variable_write_node().as_ref())
            }
            Node::HashNode { .. } => {
                hash_node::build_node(self.as_hash_node().as_ref())
            }
            Node::HashPatternNode { .. } => {
                hash_pattern_node::build_node(self.as_hash_pattern_node().as_ref())
            }
            Node::IfNode { .. } => {
                if_node::build_node(self.as_if_node().as_ref())
            }
            Node::ImaginaryNode { .. } => {
                imaginary_node::build_node(self.as_imaginary_node().as_ref())
            }
            Node::ImplicitNode { .. } => {
                implicit_node::build_node(self.as_implicit_node().as_ref())
            }
            Node::ImplicitRestNode { .. } => {
                implicit_rest_node::build_node(self.as_implicit_rest_node().as_ref())
            }
            Node::InNode { .. } => {
                in_node::build_node(self.as_in_node().as_ref())
            }
            Node::IndexAndWriteNode { .. } => {
                index_and_write_node::build_node(self.as_index_and_write_node().as_ref())
            }
            Node::IndexOperatorWriteNode { .. } => {
                index_operator_write_node::build_node(self.as_index_operator_write_node().as_ref())
            }
            Node::IndexOrWriteNode { .. } => {
                index_or_write_node::build_node(self.as_index_or_write_node().as_ref())
            }
            Node::IndexTargetNode { .. } => {
                index_target_node::build_node(self.as_index_target_node().as_ref())
            }
            Node::InstanceVariableAndWriteNode { .. } => {
                instance_variable_and_write_node::build_node(self.as_instance_variable_and_write_node().as_ref())
            }
            Node::InstanceVariableOperatorWriteNode { .. } => {
                instance_variable_operator_write_node::build_node(self.as_instance_variable_operator_write_node().as_ref())
            }
            Node::InstanceVariableOrWriteNode { .. } => {
                instance_variable_or_write_node::build_node(self.as_instance_variable_or_write_node().as_ref())
            }
            Node::InstanceVariableReadNode { .. } => {
                instance_variable_read_node::build_node(self.as_instance_variable_read_node().as_ref())
            }
            Node::InstanceVariableTargetNode { .. } => {
                instance_variable_target_node::build_node(self.as_instance_variable_target_node().as_ref())
            }
            Node::InstanceVariableWriteNode { .. } => {
                instance_variable_write_node::build_node(self.as_instance_variable_write_node().as_ref())
            }
            Node::IntegerNode { .. } => {
                integer_node::build_node(self.as_integer_node().as_ref())
            }
            Node::InterpolatedMatchLastLineNode { .. } => {
                interpolated_match_last_line_node::build_node(self.as_interpolated_match_last_line_node().as_ref())
            }
            Node::InterpolatedRegularExpressionNode { .. } => {
                interpolated_regular_expression_node::build_node(self.as_interpolated_regular_expression_node().as_ref())
            }
            Node::InterpolatedStringNode { .. } => {
                interpolated_string_node::build_node(self.as_interpolated_string_node().as_ref())
            }
            Node::InterpolatedSymbolNode { .. } => {
                interpolated_symbol_node::build_node(self.as_interpolated_symbol_node().as_ref())
            }
            Node::InterpolatedXStringNode { .. } => {
                interpolated_x_string_node::build_node(self.as_interpolated_x_string_node().as_ref())
            }
            Node::ItLocalVariableReadNode { .. } => {
                it_local_variable_read_node::build_node(self.as_it_local_variable_read_node().as_ref())
            }
            Node::ItParametersNode { .. } => {
                it_parameters_node::build_node(self.as_it_parameters_node().as_ref())
            }
            Node::KeywordHashNode { .. } => {
                keyword_hash_node::build_node(self.as_keyword_hash_node().as_ref())
            }
            Node::KeywordRestParameterNode { .. } => {
                keyword_rest_parameter_node::build_node(self.as_keyword_rest_parameter_node().as_ref())
            }
            Node::LambdaNode { .. } => {
                lambda_node::build_node(self.as_lambda_node().as_ref())
            }
            Node::LocalVariableAndWriteNode { .. } => {
                local_variable_and_write_node::build_node(self.as_local_variable_and_write_node().as_ref())
            }
            Node::LocalVariableOperatorWriteNode { .. } => {
                local_variable_operator_write_node::build_node(self.as_local_variable_operator_write_node().as_ref())
            }
            Node::LocalVariableOrWriteNode { .. } => {
                local_variable_or_write_node::build_node(self.as_local_variable_or_write_node().as_ref())
            }
            Node::LocalVariableReadNode { .. } => {
                local_variable_read_node::build_node(self.as_local_variable_read_node().as_ref())
            }
            Node::LocalVariableTargetNode { .. } => {
                local_variable_target_node::build_node(self.as_local_variable_target_node().as_ref())
            }
            Node::LocalVariableWriteNode { .. } => {
                local_variable_write_node::build_node(self.as_local_variable_write_node().as_ref())
            }
            Node::MatchLastLineNode { .. } => {
                match_last_line_node::build_node(self.as_match_last_line_node().as_ref())
            }
            Node::MatchPredicateNode { .. } => {
                match_predicate_node::build_node(self.as_match_predicate_node().as_ref())
            }
            Node::MatchRequiredNode { .. } => {
                match_required_node::build_node(self.as_match_required_node().as_ref())
            }
            Node::MatchWriteNode { .. } => {
                match_write_node::build_node(self.as_match_write_node().as_ref())
            }
            Node::MissingNode { .. } => {
                missing_node::build_node(self.as_missing_node().as_ref())
            }
            Node::ModuleNode { .. } => {
                module_node::build_node(self.as_module_node().as_ref())
            }
            Node::MultiTargetNode { .. } => {
                multi_target_node::build_node(self.as_multi_target_node().as_ref())
            }
            Node::MultiWriteNode { .. } => {
                multi_write_node::build_node(self.as_multi_write_node().as_ref())
            }
            Node::NextNode { .. } => {
                next_node::build_node(self.as_next_node().as_ref())
            }
            Node::NilNode { .. } => {
                nil_node::build_node(self.as_nil_node().as_ref())
            }
            Node::NoKeywordsParameterNode { .. } => {
                no_keywords_parameter_node::build_node(self.as_no_keywords_parameter_node().as_ref())
            }
            Node::NumberedParametersNode { .. } => {
                numbered_parameters_node::build_node(self.as_numbered_parameters_node().as_ref())
            }
            Node::NumberedReferenceReadNode { .. } => {
                numbered_reference_read_node::build_node(self.as_numbered_reference_read_node().as_ref())
            }
            Node::OptionalKeywordParameterNode { .. } => {
                optional_keyword_parameter_node::build_node(self.as_optional_keyword_parameter_node().as_ref())
            }
            Node::OptionalParameterNode { .. } => {
                optional_parameter_node::build_node(self.as_optional_parameter_node().as_ref())
            }
            Node::OrNode { .. } => {
                or_node::build_node(self.as_or_node().as_ref())
            }
            Node::ParametersNode { .. } => {
                parameters_node::build_node(self.as_parameters_node().as_ref())
            }
            Node::ParenthesesNode { .. } => {
                parentheses_node::build_node(self.as_parentheses_node().as_ref())
            }
            Node::PinnedExpressionNode { .. } => {
                pinned_expression_node::build_node(self.as_pinned_expression_node().as_ref())
            }
            Node::PinnedVariableNode { .. } => {
                pinned_variable_node::build_node(self.as_pinned_variable_node().as_ref())
            }
            Node::PostExecutionNode { .. } => {
                post_execution_node::build_node(self.as_post_execution_node().as_ref())
            }
            Node::PreExecutionNode { .. } => {
                pre_execution_node::build_node(self.as_pre_execution_node().as_ref())
            }
            Node::ProgramNode { .. } => {
                program_node::build_node(self.as_program_node().as_ref())
            }
            Node::RangeNode { .. } => {
                range_node::build_node(self.as_range_node().as_ref())
            }
            Node::RationalNode { .. } => {
                rational_node::build_node(self.as_rational_node().as_ref())
            }
            Node::RedoNode { .. } => {
                redo_node::build_node(self.as_redo_node().as_ref())
            }
            Node::RegularExpressionNode { .. } => {
                regular_expression_node::build_node(self.as_regular_expression_node().as_ref())
            }
            Node::RequiredKeywordParameterNode { .. } => {
                required_keyword_parameter_node::build_node(self.as_required_keyword_parameter_node().as_ref())
            }
            Node::RequiredParameterNode { .. } => {
                required_parameter_node::build_node(self.as_required_parameter_node().as_ref())
            }
            Node::RescueModifierNode { .. } => {
                rescue_modifier_node::build_node(self.as_rescue_modifier_node().as_ref())
            }
            Node::RescueNode { .. } => {
                rescue_node::build_node(self.as_rescue_node().as_ref())
            }
            Node::RestParameterNode { .. } => {
                rest_parameter_node::build_node(self.as_rest_parameter_node().as_ref())
            }
            Node::RetryNode { .. } => {
                retry_node::build_node(self.as_retry_node().as_ref())
            }
            Node::ReturnNode { .. } => {
                return_node::build_node(self.as_return_node().as_ref())
            }
            Node::SelfNode { .. } => {
                self_node::build_node(self.as_self_node().as_ref())
            }
            Node::ShareableConstantNode { .. } => {
                shareable_constant_node::build_node(self.as_shareable_constant_node().as_ref())
            }
            Node::SingletonClassNode { .. } => {
                singleton_class_node::build_node(self.as_singleton_class_node().as_ref())
            }
            Node::SourceEncodingNode { .. } => {
                source_encoding_node::build_node(self.as_source_encoding_node().as_ref())
            }
            Node::SourceFileNode { .. } => {
                source_file_node::build_node(self.as_source_file_node().as_ref())
            }
            Node::SourceLineNode { .. } => {
                source_line_node::build_node(self.as_source_line_node().as_ref())
            }
            Node::SplatNode { .. } => {
                splat_node::build_node(self.as_splat_node().as_ref())
            }
            Node::StatementsNode { .. } => {
                statements_node::build_node(self.as_statements_node().as_ref())
            }
            Node::StringNode { .. } => {
                string_node::build_node(self.as_string_node().as_ref())
            }
            Node::SuperNode { .. } => {
                super_node::build_node(self.as_super_node().as_ref())
            }
            Node::SymbolNode { .. } => {
                symbol_node::build_node(self.as_symbol_node().as_ref())
            }
            Node::TrueNode { .. } => {
                true_node::build_node(self.as_true_node().as_ref())
            }
            Node::UndefNode { .. } => {
                undef_node::build_node(self.as_undef_node().as_ref())
            }
            Node::UnlessNode { .. } => {
                unless_node::build_node(self.as_unless_node().as_ref())
            }
            Node::UntilNode { .. } => {
                until_node::build_node(self.as_until_node().as_ref())
            }
            Node::WhenNode { .. } => {
                when_node::build_node(self.as_when_node().as_ref())
            }
            Node::WhileNode { .. } => {
                while_node::build_node(self.as_while_node().as_ref())
            }
            Node::XStringNode { .. } => {
                x_string_node::build_node(self.as_x_string_node().as_ref())
            }
            Node::YieldNode { .. } => {
                yield_node::build_node(self.as_yield_node().as_ref())
            }
        }
    }
}
