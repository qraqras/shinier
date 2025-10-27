use crate::builder::prism_node::node::*;
use crate::document::Document;
use ruby_prism::{Comments, Node};
use std::collections::HashMap;

pub trait BuildPrismNode {
    //
    #[rustfmt::skip]
    fn _build(&self, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document;
    //
    #[rustfmt::skip]
    fn build(&self, comments: &mut Comments) -> Document {
        self._build(comments, None)
    }
    //
    #[rustfmt::skip]
    fn build_with(
        &self, comments: &mut Comments, before: Option<Document>, after: Option<Document>) -> Document {
        let before = before.unwrap_or(Document::None);
        let after = after.unwrap_or(Document::None);
        Document::Array(Vec::from([before, self.build(comments), after]))
    }
    //
    #[rustfmt::skip]
    fn build_optional(&self, comments: &mut Comments, option: &HashMap<&str, bool>) -> Document {
        self._build(comments, Some(option))
    }
    //
    #[rustfmt::skip]
    fn build_optional_with(&self, comments: &mut Comments, option: &HashMap<&str, bool>, before: Document, after: Document) -> Document {
        Document::Array(Vec::from([before, self.build_optional(comments, option), after]))
    }
}

impl BuildPrismNode for Node<'_> {
    #[rustfmt::skip]
    fn _build(&self, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
         match self {
            Node::AliasGlobalVariableNode { .. } => {
                alias_global_variable_node::build_node(self.as_alias_global_variable_node().as_ref(), comments, option)
            }
            Node::AliasMethodNode { .. } => {
                alias_method_node::build_node(self.as_alias_method_node().as_ref(), comments, option)
            }
            Node::AlternationPatternNode { .. } => {
                alternation_pattern_node::build_node(self.as_alternation_pattern_node().as_ref(), comments, option)
            }
            Node::AndNode { .. } => {
                and_node::build_node(self.as_and_node().as_ref(), comments, option)
            }
            Node::ArgumentsNode { .. } => {
                arguments_node::build_node(self.as_arguments_node().as_ref(), comments, option)
            }
            Node::ArrayNode { .. } => {
                array_node::build_node(self.as_array_node().as_ref(), comments, option)
            }
            Node::ArrayPatternNode { .. } => {
                array_pattern_node::build_node(self.as_array_pattern_node().as_ref(), comments, option)
            }
            Node::AssocNode { .. } => {
                assoc_node::build_node(self.as_assoc_node().as_ref(), comments, option)
            }
            Node::AssocSplatNode { .. } => {
                assoc_splat_node::build_node(self.as_assoc_splat_node().as_ref(), comments, option)
            }
            Node::BackReferenceReadNode { .. } => {
                back_reference_read_node::build_node(self.as_back_reference_read_node().as_ref(), comments, option)
            }
            Node::BeginNode { .. } => {
                begin_node::build_node(self.as_begin_node().as_ref(), comments, option)
            }
            Node::BlockArgumentNode { .. } => {
                block_argument_node::build_node(self.as_block_argument_node().as_ref(), comments, option)
            }
            Node::BlockLocalVariableNode { .. } => {
                block_local_variable_node::build_node(self.as_block_local_variable_node().as_ref(), comments, option)
            }
            Node::BlockNode { .. } => {
                block_node::build_node(self.as_block_node().as_ref(), comments, option)
            }
            Node::BlockParameterNode { .. } => {
                block_parameter_node::build_node(self.as_block_parameter_node().as_ref(), comments, option)
            }
            Node::BlockParametersNode { .. } => {
                block_parameters_node::build_node(self.as_block_parameters_node().as_ref(), comments, option)
            }
            Node::BreakNode { .. } => {
                break_node::build_node(self.as_break_node().as_ref(), comments, option)
            }
            Node::CallAndWriteNode { .. } => {
                call_and_write_node::build_node(self.as_call_and_write_node().as_ref(), comments, option)
            }
            Node::CallNode { .. } => {
                call_node::build_node(self.as_call_node().as_ref(), comments, option)
            }
            Node::CallOperatorWriteNode { .. } => {
                call_operator_write_node::build_node(self.as_call_operator_write_node().as_ref(), comments, option)
            }
            Node::CallOrWriteNode { .. } => {
                call_or_write_node::build_node(self.as_call_or_write_node().as_ref(), comments, option)
            }
            Node::CallTargetNode { .. } => {
                call_target_node::build_node(self.as_call_target_node().as_ref(), comments, option)
            }
            Node::CapturePatternNode { .. } => {
                capture_pattern_node::build_node(self.as_capture_pattern_node().as_ref(), comments, option)
            }
            Node::CaseMatchNode { .. } => {
                case_match_node::build_node(self.as_case_match_node().as_ref(), comments, option)
            }
            Node::CaseNode { .. } => {
                case_node::build_node(self.as_case_node().as_ref(), comments, option)
            }
            Node::ClassNode { .. } => {
                class_node::build_node(self.as_class_node().as_ref(), comments, option)
            }
            Node::ClassVariableAndWriteNode { .. } => {
                class_variable_and_write_node::build_node(self.as_class_variable_and_write_node().as_ref(), comments, option)
            }
            Node::ClassVariableOperatorWriteNode { .. } => {
                class_variable_operator_write_node::build_node(self.as_class_variable_operator_write_node().as_ref(), comments, option)
            }
            Node::ClassVariableOrWriteNode { .. } => {
                class_variable_or_write_node::build_node(self.as_class_variable_or_write_node().as_ref(), comments, option)
            }
            Node::ClassVariableReadNode { .. } => {
                class_variable_read_node::build_node(self.as_class_variable_read_node().as_ref(), comments, option)
            }
            Node::ClassVariableTargetNode { .. } => {
                class_variable_target_node::build_node(self.as_class_variable_target_node().as_ref(), comments, option)
            }
            Node::ClassVariableWriteNode { .. } => {
                class_variable_write_node::build_node(self.as_class_variable_write_node().as_ref(), comments, option)
            }
            Node::ConstantAndWriteNode { .. } => {
                constant_and_write_node::build_node(self.as_constant_and_write_node().as_ref(), comments, option)
            }
            Node::ConstantOperatorWriteNode { .. } => {
                constant_operator_write_node::build_node(self.as_constant_operator_write_node().as_ref(), comments, option)
            }
            Node::ConstantOrWriteNode { .. } => {
                constant_or_write_node::build_node(self.as_constant_or_write_node().as_ref(), comments, option)
            }
            Node::ConstantPathAndWriteNode { .. } => {
                constant_path_and_write_node::build_node(self.as_constant_path_and_write_node().as_ref(), comments, option)
            }
            Node::ConstantPathNode { .. } => {
                constant_path_node::build_node(self.as_constant_path_node().as_ref(), comments, option)
            }
            Node::ConstantPathOperatorWriteNode { .. } => {
                constant_path_operator_write_node::build_node(self.as_constant_path_operator_write_node().as_ref(), comments, option)
            }
            Node::ConstantPathOrWriteNode { .. } => {
                constant_path_or_write_node::build_node(self.as_constant_path_or_write_node().as_ref(), comments, option)
            }
            Node::ConstantPathTargetNode { .. } => {
                constant_path_target_node::build_node(self.as_constant_path_target_node().as_ref(), comments, option)
            }
            Node::ConstantPathWriteNode { .. } => {
                constant_path_write_node::build_node(self.as_constant_path_write_node().as_ref(), comments, option)
            }
            Node::ConstantReadNode { .. } => {
                constant_read_node::build_node(self.as_constant_read_node().as_ref(), comments, option)
            }
            Node::ConstantTargetNode { .. } => {
                constant_target_node::build_node(self.as_constant_target_node().as_ref(), comments, option)
            }
            Node::ConstantWriteNode { .. } => {
                constant_write_node::build_node(self.as_constant_write_node().as_ref(), comments, option)
            }
            Node::DefNode { .. } => {
                def_node::build_node(self.as_def_node().as_ref(), comments, option)
            }
            Node::DefinedNode { .. } => {
                defined_node::build_node(self.as_defined_node().as_ref(), comments, option)
            }
            Node::ElseNode { .. } => {
                else_node::build_node(self.as_else_node().as_ref(), comments, option)
            }
            Node::EmbeddedStatementsNode { .. } => {
                embedded_statements_node::build_node(self.as_embedded_statements_node().as_ref(), comments, option)
            }
            Node::EmbeddedVariableNode { .. } => {
                embedded_variable_node::build_node(self.as_embedded_variable_node().as_ref(), comments, option)
            }
            Node::EnsureNode { .. } => {
                ensure_node::build_node(self.as_ensure_node().as_ref(), comments, option)
            }
            Node::FalseNode { .. } => {
                false_node::build_node(self.as_false_node().as_ref(), comments, option)
            }
            Node::FindPatternNode { .. } => {
                find_pattern_node::build_node(self.as_find_pattern_node().as_ref(), comments, option)
            }
            Node::FlipFlopNode { .. } => {
                flip_flop_node::build_node(self.as_flip_flop_node().as_ref(), comments, option)
            }
            Node::FloatNode { .. } => {
                float_node::build_node(self.as_float_node().as_ref(), comments, option)
            }
            Node::ForNode { .. } => {
                for_node::build_node(self.as_for_node().as_ref(), comments, option)
            }
            Node::ForwardingArgumentsNode { .. } => {
                forwarding_arguments_node::build_node(self.as_forwarding_arguments_node().as_ref(), comments, option)
            }
            Node::ForwardingParameterNode { .. } => {
                forwarding_parameter_node::build_node(self.as_forwarding_parameter_node().as_ref(), comments, option)
            }
            Node::ForwardingSuperNode { .. } => {
                forwarding_super_node::build_node(self.as_forwarding_super_node().as_ref(), comments, option)
            }
            Node::GlobalVariableAndWriteNode { .. } => {
                global_variable_and_write_node::build_node(self.as_global_variable_and_write_node().as_ref(), comments, option)
            }
            Node::GlobalVariableOperatorWriteNode { .. } => {
                global_variable_operator_write_node::build_node(self.as_global_variable_operator_write_node().as_ref(), comments, option)
            }
            Node::GlobalVariableOrWriteNode { .. } => {
                global_variable_or_write_node::build_node(self.as_global_variable_or_write_node().as_ref(), comments, option)
            }
            Node::GlobalVariableReadNode { .. } => {
                global_variable_read_node::build_node(self.as_global_variable_read_node().as_ref(), comments, option)
            }
            Node::GlobalVariableTargetNode { .. } => {
                global_variable_target_node::build_node(self.as_global_variable_target_node().as_ref(), comments, option)
            }
            Node::GlobalVariableWriteNode { .. } => {
                global_variable_write_node::build_node(self.as_global_variable_write_node().as_ref(), comments, option)
            }
            Node::HashNode { .. } => {
                hash_node::build_node(self.as_hash_node().as_ref(), comments, option)
            }
            Node::HashPatternNode { .. } => {
                hash_pattern_node::build_node(self.as_hash_pattern_node().as_ref(), comments, option)
            }
            Node::IfNode { .. } => {
                if_node::build_node(self.as_if_node().as_ref(), comments, option)
            }
            Node::ImaginaryNode { .. } => {
                imaginary_node::build_node(self.as_imaginary_node().as_ref(), comments, option)
            }
            Node::ImplicitNode { .. } => {
                implicit_node::build_node(self.as_implicit_node().as_ref(), comments, option)
            }
            Node::ImplicitRestNode { .. } => {
                implicit_rest_node::build_node(self.as_implicit_rest_node().as_ref(), comments, option)
            }
            Node::InNode { .. } => {
                in_node::build_node(self.as_in_node().as_ref(), comments, option)
            }
            Node::IndexAndWriteNode { .. } => {
                index_and_write_node::build_node(self.as_index_and_write_node().as_ref(), comments, option)
            }
            Node::IndexOperatorWriteNode { .. } => {
                index_operator_write_node::build_node(self.as_index_operator_write_node().as_ref(), comments, option)
            }
            Node::IndexOrWriteNode { .. } => {
                index_or_write_node::build_node(self.as_index_or_write_node().as_ref(), comments, option)
            }
            Node::IndexTargetNode { .. } => {
                index_target_node::build_node(self.as_index_target_node().as_ref(), comments, option)
            }
            Node::InstanceVariableAndWriteNode { .. } => {
                instance_variable_and_write_node::build_node(self.as_instance_variable_and_write_node().as_ref(), comments, option)
            }
            Node::InstanceVariableOperatorWriteNode { .. } => {
                instance_variable_operator_write_node::build_node(self.as_instance_variable_operator_write_node().as_ref(), comments, option)
            }
            Node::InstanceVariableOrWriteNode { .. } => {
                instance_variable_or_write_node::build_node(self.as_instance_variable_or_write_node().as_ref(), comments, option)
            }
            Node::InstanceVariableReadNode { .. } => {
                instance_variable_read_node::build_node(self.as_instance_variable_read_node().as_ref(), comments, option)
            }
            Node::InstanceVariableTargetNode { .. } => {
                instance_variable_target_node::build_node(self.as_instance_variable_target_node().as_ref(), comments, option)
            }
            Node::InstanceVariableWriteNode { .. } => {
                instance_variable_write_node::build_node(self.as_instance_variable_write_node().as_ref(), comments, option)
            }
            Node::IntegerNode { .. } => {
                integer_node::build_node(self.as_integer_node().as_ref(), comments, option)
            }
            Node::InterpolatedMatchLastLineNode { .. } => {
                interpolated_match_last_line_node::build_node(self.as_interpolated_match_last_line_node().as_ref(), comments, option)
            }
            Node::InterpolatedRegularExpressionNode { .. } => {
                interpolated_regular_expression_node::build_node(self.as_interpolated_regular_expression_node().as_ref(), comments, option)
            }
            Node::InterpolatedStringNode { .. } => {
                interpolated_string_node::build_node(self.as_interpolated_string_node().as_ref(), comments, option)
            }
            Node::InterpolatedSymbolNode { .. } => {
                interpolated_symbol_node::build_node(self.as_interpolated_symbol_node().as_ref(), comments, option)
            }
            Node::InterpolatedXStringNode { .. } => {
                interpolated_x_string_node::build_node(self.as_interpolated_x_string_node().as_ref(), comments, option)
            }
            Node::ItLocalVariableReadNode { .. } => {
                it_local_variable_read_node::build_node(self.as_it_local_variable_read_node().as_ref(), comments, option)
            }
            Node::ItParametersNode { .. } => {
                it_parameters_node::build_node(self.as_it_parameters_node().as_ref(), comments, option)
            }
            Node::KeywordHashNode { .. } => {
                keyword_hash_node::build_node(self.as_keyword_hash_node().as_ref(), comments, option)
            }
            Node::KeywordRestParameterNode { .. } => {
                keyword_rest_parameter_node::build_node(self.as_keyword_rest_parameter_node().as_ref(), comments, option)
            }
            Node::LambdaNode { .. } => {
                lambda_node::build_node(self.as_lambda_node().as_ref(), comments, option)
            }
            Node::LocalVariableAndWriteNode { .. } => {
                local_variable_and_write_node::build_node(self.as_local_variable_and_write_node().as_ref(), comments, option)
            }
            Node::LocalVariableOperatorWriteNode { .. } => {
                local_variable_operator_write_node::build_node(self.as_local_variable_operator_write_node().as_ref(), comments, option)
            }
            Node::LocalVariableOrWriteNode { .. } => {
                local_variable_or_write_node::build_node(self.as_local_variable_or_write_node().as_ref(), comments, option)
            }
            Node::LocalVariableReadNode { .. } => {
                local_variable_read_node::build_node(self.as_local_variable_read_node().as_ref(), comments, option)
            }
            Node::LocalVariableTargetNode { .. } => {
                local_variable_target_node::build_node(self.as_local_variable_target_node().as_ref(), comments, option)
            }
            Node::LocalVariableWriteNode { .. } => {
                local_variable_write_node::build_node(self.as_local_variable_write_node().as_ref(), comments, option)
            }
            Node::MatchLastLineNode { .. } => {
                match_last_line_node::build_node(self.as_match_last_line_node().as_ref(), comments, option)
            }
            Node::MatchPredicateNode { .. } => {
                match_predicate_node::build_node(self.as_match_predicate_node().as_ref(), comments, option)
            }
            Node::MatchRequiredNode { .. } => {
                match_required_node::build_node(self.as_match_required_node().as_ref(), comments, option)
            }
            Node::MatchWriteNode { .. } => {
                match_write_node::build_node(self.as_match_write_node().as_ref(), comments, option)
            }
            Node::MissingNode { .. } => {
                missing_node::build_node(self.as_missing_node().as_ref(), comments, option)
            }
            Node::ModuleNode { .. } => {
                module_node::build_node(self.as_module_node().as_ref(), comments, option)
            }
            Node::MultiTargetNode { .. } => {
                multi_target_node::build_node(self.as_multi_target_node().as_ref(), comments, option)
            }
            Node::MultiWriteNode { .. } => {
                multi_write_node::build_node(self.as_multi_write_node().as_ref(), comments, option)
            }
            Node::NextNode { .. } => {
                next_node::build_node(self.as_next_node().as_ref(), comments, option)
            }
            Node::NilNode { .. } => {
                nil_node::build_node(self.as_nil_node().as_ref(), comments, option)
            }
            Node::NoKeywordsParameterNode { .. } => {
                no_keywords_parameter_node::build_node(self.as_no_keywords_parameter_node().as_ref(), comments, option)
            }
            Node::NumberedParametersNode { .. } => {
                numbered_parameters_node::build_node(self.as_numbered_parameters_node().as_ref(), comments, option)
            }
            Node::NumberedReferenceReadNode { .. } => {
                numbered_reference_read_node::build_node(self.as_numbered_reference_read_node().as_ref(), comments, option)
            }
            Node::OptionalKeywordParameterNode { .. } => {
                optional_keyword_parameter_node::build_node(self.as_optional_keyword_parameter_node().as_ref(), comments, option)
            }
            Node::OptionalParameterNode { .. } => {
                optional_parameter_node::build_node(self.as_optional_parameter_node().as_ref(), comments, option)
            }
            Node::OrNode { .. } => {
                or_node::build_node(self.as_or_node().as_ref(), comments, option)
            }
            Node::ParametersNode { .. } => {
                parameters_node::build_node(self.as_parameters_node().as_ref(), comments, option)
            }
            Node::ParenthesesNode { .. } => {
                parentheses_node::build_node(self.as_parentheses_node().as_ref(), comments, option)
            }
            Node::PinnedExpressionNode { .. } => {
                pinned_expression_node::build_node(self.as_pinned_expression_node().as_ref(), comments, option)
            }
            Node::PinnedVariableNode { .. } => {
                pinned_variable_node::build_node(self.as_pinned_variable_node().as_ref(), comments, option)
            }
            Node::PostExecutionNode { .. } => {
                post_execution_node::build_node(self.as_post_execution_node().as_ref(), comments, option)
            }
            Node::PreExecutionNode { .. } => {
                pre_execution_node::build_node(self.as_pre_execution_node().as_ref(), comments, option)
            }
            Node::ProgramNode { .. } => {
                program_node::build_node(self.as_program_node().as_ref(), comments, option)
            }
            Node::RangeNode { .. } => {
                range_node::build_node(self.as_range_node().as_ref(), comments, option)
            }
            Node::RationalNode { .. } => {
                rational_node::build_node(self.as_rational_node().as_ref(), comments, option)
            }
            Node::RedoNode { .. } => {
                redo_node::build_node(self.as_redo_node().as_ref(), comments, option)
            }
            Node::RegularExpressionNode { .. } => {
                regular_expression_node::build_node(self.as_regular_expression_node().as_ref(), comments, option)
            }
            Node::RequiredKeywordParameterNode { .. } => {
                required_keyword_parameter_node::build_node(self.as_required_keyword_parameter_node().as_ref(), comments, option)
            }
            Node::RequiredParameterNode { .. } => {
                required_parameter_node::build_node(self.as_required_parameter_node().as_ref(), comments, option)
            }
            Node::RescueModifierNode { .. } => {
                rescue_modifier_node::build_node(self.as_rescue_modifier_node().as_ref(), comments, option)
            }
            Node::RescueNode { .. } => {
                rescue_node::build_node(self.as_rescue_node().as_ref(), comments, option)
            }
            Node::RestParameterNode { .. } => {
                rest_parameter_node::build_node(self.as_rest_parameter_node().as_ref(), comments, option)
            }
            Node::RetryNode { .. } => {
                retry_node::build_node(self.as_retry_node().as_ref(), comments, option)
            }
            Node::ReturnNode { .. } => {
                return_node::build_node(self.as_return_node().as_ref(), comments, option)
            }
            Node::SelfNode { .. } => {
                self_node::build_node(self.as_self_node().as_ref(), comments, option)
            }
            Node::ShareableConstantNode { .. } => {
                shareable_constant_node::build_node(self.as_shareable_constant_node().as_ref(), comments, option)
            }
            Node::SingletonClassNode { .. } => {
                singleton_class_node::build_node(self.as_singleton_class_node().as_ref(), comments, option)
            }
            Node::SourceEncodingNode { .. } => {
                source_encoding_node::build_node(self.as_source_encoding_node().as_ref(), comments, option)
            }
            Node::SourceFileNode { .. } => {
                source_file_node::build_node(self.as_source_file_node().as_ref(), comments, option)
            }
            Node::SourceLineNode { .. } => {
                source_line_node::build_node(self.as_source_line_node().as_ref(), comments, option)
            }
            Node::SplatNode { .. } => {
                splat_node::build_node(self.as_splat_node().as_ref(), comments, option)
            }
            Node::StatementsNode { .. } => {
                statements_node::build_node(self.as_statements_node().as_ref(), comments, option)
            }
            Node::StringNode { .. } => {
                string_node::build_node(self.as_string_node().as_ref(), comments, option)
            }
            Node::SuperNode { .. } => {
                super_node::build_node(self.as_super_node().as_ref(), comments, option)
            }
            Node::SymbolNode { .. } => {
                symbol_node::build_node(self.as_symbol_node().as_ref(), comments, option)
            }
            Node::TrueNode { .. } => {
                true_node::build_node(self.as_true_node().as_ref(), comments, option)
            }
            Node::UndefNode { .. } => {
                undef_node::build_node(self.as_undef_node().as_ref(), comments, option)
            }
            Node::UnlessNode { .. } => {
                unless_node::build_node(self.as_unless_node().as_ref(), comments, option)
            }
            Node::UntilNode { .. } => {
                until_node::build_node(self.as_until_node().as_ref(), comments, option)
            }
            Node::WhenNode { .. } => {
                when_node::build_node(self.as_when_node().as_ref(), comments, option)
            }
            Node::WhileNode { .. } => {
                while_node::build_node(self.as_while_node().as_ref(), comments, option)
            }
            Node::XStringNode { .. } => {
                x_string_node::build_node(self.as_x_string_node().as_ref(), comments, option)
            }
            Node::YieldNode { .. } => {
                yield_node::build_node(self.as_yield_node().as_ref(), comments, option)
            }
        }
    }
}

impl BuildPrismNode for Option<Node<'_>> {
    fn _build(&self, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
        match self {
            Some(node) => node._build(comments, option),
            None => Document::None,
        }
    }
}
