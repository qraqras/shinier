use crate::builder::node::*;
use crate::doc::*;
use ruby_prism::*;

#[rustfmt::skip]
pub fn build(node: &Node) -> Doc {
    match node {
        Node::AliasGlobalVariableNode { .. } => {
            alias_global_variable_node::build_node(&node.as_alias_global_variable_node().unwrap())
        }
        Node::AliasMethodNode { .. } => {
            alias_method_node::build_node(&node.as_alias_method_node().unwrap())
        }
        Node::AlternationPatternNode { .. } => {
            alternation_pattern_node::build_node(&node.as_alternation_pattern_node().unwrap())
        }
        Node::AndNode { .. } => {
            and_node::build_node(&node.as_and_node().unwrap())
        }
        Node::ArgumentsNode { .. } => {
            arguments_node::build_node(&node.as_arguments_node().unwrap())
        }
        Node::ArrayNode { .. } => {
            array_node::build_node(&node.as_array_node().unwrap())
        }
        Node::ArrayPatternNode { .. } => {
            array_pattern_node::build_node(&node.as_array_pattern_node().unwrap())
        }
        Node::AssocNode { .. } => {
            assoc_node::build_node(&node.as_assoc_node().unwrap())
        }
        Node::AssocSplatNode { .. } => {
            assoc_splat_node::build_node(&node.as_assoc_splat_node().unwrap())
        }
        Node::BackReferenceReadNode { .. } => {
            back_reference_read_node::build_node(&node.as_back_reference_read_node().unwrap())
        }
        Node::BeginNode { .. } => {
            begin_node::build_node(&node.as_begin_node().unwrap())
        }
        Node::BlockArgumentNode { .. } => {
            block_argument_node::build_node(&node.as_block_argument_node().unwrap())
        }
        Node::BlockLocalVariableNode { .. } => {
            block_local_variable_node::build_node(&node.as_block_local_variable_node().unwrap())
        }
        Node::BlockNode { .. } => {
            block_node::build_node(&node.as_block_node().unwrap())
        }
        Node::BlockParameterNode { .. } => {
            block_parameter_node::build_node(&node.as_block_parameter_node().unwrap())
        }
        Node::BlockParametersNode { .. } => {
            block_parameters_node::build_node(&node.as_block_parameters_node().unwrap())
        }
        Node::BreakNode { .. } => {
            break_node::build_node(&node.as_break_node().unwrap())
        }
        Node::CallAndWriteNode { .. } => {
            call_and_write_node::build_node(&node.as_call_and_write_node().unwrap())
        }
        Node::CallNode { .. } => {
            call_node::build_node(&node.as_call_node().unwrap())
        }
        Node::CallOperatorWriteNode { .. } => {
            call_operator_write_node::build_node(&node.as_call_operator_write_node().unwrap())
        }
        Node::CallOrWriteNode { .. } => {
            call_or_write_node::build_node(&node.as_call_or_write_node().unwrap())
        }
        Node::CallTargetNode { .. } => {
            call_target_node::build_node(&node.as_call_target_node().unwrap())
        }
        Node::CapturePatternNode { .. } => {
            capture_pattern_node::build_node(&node.as_capture_pattern_node().unwrap())
        }
        Node::CaseMatchNode { .. } => {
            case_match_node::build_node(&node.as_case_match_node().unwrap())
        }
        Node::CaseNode { .. } => {
            case_node::build_node(&node.as_case_node().unwrap())
        }
        Node::ClassNode { .. } => {
            class_node::build_node(&node.as_class_node().unwrap())
        }
        Node::ClassVariableAndWriteNode { .. } => {
            class_variable_and_write_node::build_node(&node.as_class_variable_and_write_node().unwrap())
        }
        Node::ClassVariableOperatorWriteNode { .. } => {
            class_variable_operator_write_node::build_node(&node.as_class_variable_operator_write_node().unwrap())
        }
        Node::ClassVariableOrWriteNode { .. } => {
            class_variable_or_write_node::build_node(&node.as_class_variable_or_write_node().unwrap())
        }
        Node::ClassVariableReadNode { .. } => {
            class_variable_read_node::build_node(&node.as_class_variable_read_node().unwrap())
        }
        Node::ClassVariableTargetNode { .. } => {
            class_variable_target_node::build_node(&node.as_class_variable_target_node().unwrap())
        }
        Node::ClassVariableWriteNode { .. } => {
            class_variable_write_node::build_node(&node.as_class_variable_write_node().unwrap())
        }
        Node::ConstantAndWriteNode { .. } => {
            constant_and_write_node::build_node(&node.as_constant_and_write_node().unwrap())
        }
        Node::ConstantOperatorWriteNode { .. } => {
            constant_operator_write_node::build_node(&node.as_constant_operator_write_node().unwrap())
        }
        Node::ConstantOrWriteNode { .. } => {
            constant_or_write_node::build_node(&node.as_constant_or_write_node().unwrap())
        }
        Node::ConstantPathAndWriteNode { .. } => {
            constant_path_and_write_node::build_node(&node.as_constant_path_and_write_node().unwrap())
        }
        Node::ConstantPathNode { .. } => {
            constant_path_node::build_node(&node.as_constant_path_node().unwrap())
        }
        Node::ConstantPathOperatorWriteNode { .. } => {
            constant_path_operator_write_node::build_node(&node.as_constant_path_operator_write_node().unwrap())
        }
        Node::ConstantPathOrWriteNode { .. } => {
            constant_path_or_write_node::build_node(&node.as_constant_path_or_write_node().unwrap())
        }
        Node::ConstantPathTargetNode { .. } => {
            constant_path_target_node::build_node(&node.as_constant_path_target_node().unwrap())
        }
        Node::ConstantPathWriteNode { .. } => {
            constant_path_write_node::build_node(&node.as_constant_path_write_node().unwrap())
        }
        Node::ConstantReadNode { .. } => {
            constant_read_node::build_node(&node.as_constant_read_node().unwrap())
        }
        Node::ConstantTargetNode { .. } => {
            constant_target_node::build_node(&node.as_constant_target_node().unwrap())
        }
        Node::ConstantWriteNode { .. } => {
            constant_write_node::build_node(&node.as_constant_write_node().unwrap())
        }
        Node::DefNode { .. } => {
            def_node::build_node(&node.as_def_node().unwrap())
        }
        Node::DefinedNode { .. } => {
            defined_node::build_node(&node.as_defined_node().unwrap())
        }
        Node::ElseNode { .. } => {
            else_node::build_node(&node.as_else_node().unwrap())
        }
        Node::EmbeddedStatementsNode { .. } => {
            embedded_statements_node::build_node(&node.as_embedded_statements_node().unwrap())
        }
        Node::EmbeddedVariableNode { .. } => {
            embedded_variable_node::build_node(&node.as_embedded_variable_node().unwrap())
        }
        Node::EnsureNode { .. } => {
            ensure_node::build_node(&node.as_ensure_node().unwrap())
        }
        Node::FalseNode { .. } => {
            false_node::build_node(&node.as_false_node().unwrap())
        }
        Node::FindPatternNode { .. } => {
            find_pattern_node::build_node(&node.as_find_pattern_node().unwrap())
        }
        Node::FlipFlopNode { .. } => {
            flip_flop_node::build_node(&node.as_flip_flop_node().unwrap())
        }
        Node::FloatNode { .. } => {
            float_node::build_node(&node.as_float_node().unwrap())
        }
        Node::ForNode { .. } => {
            for_node::build_node(&node.as_for_node().unwrap())
        }
        Node::ForwardingArgumentsNode { .. } => {
            forwarding_arguments_node::build_node(&node.as_forwarding_arguments_node().unwrap())
        }
        Node::ForwardingParameterNode { .. } => {
            forwarding_parameter_node::build_node(&node.as_forwarding_parameter_node().unwrap())
        }
        Node::ForwardingSuperNode { .. } => {
            forwarding_super_node::build_node(&node.as_forwarding_super_node().unwrap())
        }
        Node::GlobalVariableAndWriteNode { .. } => {
            global_variable_and_write_node::build_node(&node.as_global_variable_and_write_node().unwrap())
        }
        Node::GlobalVariableOperatorWriteNode { .. } => {
            global_variable_operator_write_node::build_node(&node.as_global_variable_operator_write_node().unwrap())
        }
        Node::GlobalVariableOrWriteNode { .. } => {
            global_variable_or_write_node::build_node(&node.as_global_variable_or_write_node().unwrap())
        }
        Node::GlobalVariableReadNode { .. } => {
            global_variable_read_node::build_node(&node.as_global_variable_read_node().unwrap())
        }
        Node::GlobalVariableTargetNode { .. } => {
            global_variable_target_node::build_node(&node.as_global_variable_target_node().unwrap())
        }
        Node::GlobalVariableWriteNode { .. } => {
            global_variable_write_node::build_node(&node.as_global_variable_write_node().unwrap())
        }
        Node::HashNode { .. } => {
            hash_node::build_node(&node.as_hash_node().unwrap())
        }
        Node::HashPatternNode { .. } => {
            hash_pattern_node::build_node(&node.as_hash_pattern_node().unwrap())
        }
        Node::IfNode { .. } => {
            if_node::build_node(&node.as_if_node().unwrap())
        }
        Node::ImaginaryNode { .. } => {
            imaginary_node::build_node(&node.as_imaginary_node().unwrap())
        }
        Node::ImplicitNode { .. } => {
            implicit_node::build_node(&node.as_implicit_node().unwrap())
        }
        Node::ImplicitRestNode { .. } => {
            implicit_rest_node::build_node(&node.as_implicit_rest_node().unwrap())
        }
        Node::InNode { .. } => {
            in_node::build_node(&node.as_in_node().unwrap())
        }
        Node::IndexAndWriteNode { .. } => {
            index_and_write_node::build_node(&node.as_index_and_write_node().unwrap())
        }
        Node::IndexOperatorWriteNode { .. } => {
            index_operator_write_node::build_node(&node.as_index_operator_write_node().unwrap())
        }
        Node::IndexOrWriteNode { .. } => {
            index_or_write_node::build_node(&node.as_index_or_write_node().unwrap())
        }
        Node::IndexTargetNode { .. } => {
            index_target_node::build_node(&node.as_index_target_node().unwrap())
        }
        Node::InstanceVariableAndWriteNode { .. } => {
            instance_variable_and_write_node::build_node(&node.as_instance_variable_and_write_node().unwrap())
        }
        Node::InstanceVariableOperatorWriteNode { .. } => {
            instance_variable_operator_write_node::build_node(&node.as_instance_variable_operator_write_node().unwrap())
        }
        Node::InstanceVariableOrWriteNode { .. } => {
            instance_variable_or_write_node::build_node(&node.as_instance_variable_or_write_node().unwrap())
        }
        Node::InstanceVariableReadNode { .. } => {
            instance_variable_read_node::build_node(&node.as_instance_variable_read_node().unwrap())
        }
        Node::InstanceVariableTargetNode { .. } => {
            instance_variable_target_node::build_node(&node.as_instance_variable_target_node().unwrap())
        }
        Node::InstanceVariableWriteNode { .. } => {
            instance_variable_write_node::build_node(&node.as_instance_variable_write_node().unwrap())
        }
        Node::IntegerNode { .. } => {
            integer_node::build_node(&node.as_integer_node().unwrap())
        }
        Node::InterpolatedMatchLastLineNode { .. } => {
            interpolated_match_last_line_node::build_node(&node.as_interpolated_match_last_line_node().unwrap())
        }
        Node::InterpolatedRegularExpressionNode { .. } => {
            interpolated_regular_expression_node::build_node(&node.as_interpolated_regular_expression_node().unwrap())
        }
        Node::InterpolatedStringNode { .. } => {
            interpolated_string_node::build_node(&node.as_interpolated_string_node().unwrap())
        }
        Node::InterpolatedSymbolNode { .. } => {
            interpolated_symbol_node::build_node(&node.as_interpolated_symbol_node().unwrap())
        }
        Node::InterpolatedXStringNode { .. } => {
            interpolated_x_string_node::build_node(&node.as_interpolated_x_string_node().unwrap())
        }
        Node::ItLocalVariableReadNode { .. } => {
            it_local_variable_read_node::build_node(&node.as_it_local_variable_read_node().unwrap())
        }
        Node::ItParametersNode { .. } => {
            it_parameters_node::build_node(&node.as_it_parameters_node().unwrap())
        }
        Node::KeywordHashNode { .. } => {
            keyword_hash_node::build_node(&node.as_keyword_hash_node().unwrap())
        }
        Node::KeywordRestParameterNode { .. } => {
            keyword_rest_parameter_node::build_node(&node.as_keyword_rest_parameter_node().unwrap())
        }
        Node::LambdaNode { .. } => {
            lambda_node::build_node(&node.as_lambda_node().unwrap())
        }
        Node::LocalVariableAndWriteNode { .. } => {
            local_variable_and_write_node::build_node(&node.as_local_variable_and_write_node().unwrap())
        }
        Node::LocalVariableOperatorWriteNode { .. } => {
            local_variable_operator_write_node::build_node(&node.as_local_variable_operator_write_node().unwrap())
        }
        Node::LocalVariableOrWriteNode { .. } => {
            local_variable_or_write_node::build_node(&node.as_local_variable_or_write_node().unwrap())
        }
        Node::LocalVariableReadNode { .. } => {
            local_variable_read_node::build_node(&node.as_local_variable_read_node().unwrap())
        }
        Node::LocalVariableTargetNode { .. } => {
            local_variable_target_node::build_node(&node.as_local_variable_target_node().unwrap())
        }
        Node::LocalVariableWriteNode { .. } => {
            local_variable_write_node::build_node(&node.as_local_variable_write_node().unwrap())
        }
        Node::MatchLastLineNode { .. } => {
            match_last_line_node::build_node(&node.as_match_last_line_node().unwrap())
        }
        Node::MatchPredicateNode { .. } => {
            match_predicate_node::build_node(&node.as_match_predicate_node().unwrap())
        }
        Node::MatchRequiredNode { .. } => {
            match_required_node::build_node(&node.as_match_required_node().unwrap())
        }
        Node::MatchWriteNode { .. } => {
            match_write_node::build_node(&node.as_match_write_node().unwrap())
        }
        Node::MissingNode { .. } => {
            missing_node::build_node(&node.as_missing_node().unwrap())
        }
        Node::ModuleNode { .. } => {
            module_node::build_node(&node.as_module_node().unwrap())
        }
        Node::MultiTargetNode { .. } => {
            multi_target_node::build_node(&node.as_multi_target_node().unwrap())
        }
        Node::MultiWriteNode { .. } => {
            multi_write_node::build_node(&node.as_multi_write_node().unwrap())
        }
        Node::NextNode { .. } => {
            next_node::build_node(&node.as_next_node().unwrap())
        }
        Node::NilNode { .. } => {
            nil_node::build_node(&node.as_nil_node().unwrap())
        }
        Node::NoKeywordsParameterNode { .. } => {
            no_keywords_parameter_node::build_node(&node.as_no_keywords_parameter_node().unwrap())
        }
        Node::NumberedParametersNode { .. } => {
            numbered_parameters_node::build_node(&node.as_numbered_parameters_node().unwrap())
        }
        Node::NumberedReferenceReadNode { .. } => {
            numbered_reference_read_node::build_node(&node.as_numbered_reference_read_node().unwrap())
        }
        Node::OptionalKeywordParameterNode { .. } => {
            optional_keyword_parameter_node::build_node(&node.as_optional_keyword_parameter_node().unwrap())
        }
        Node::OptionalParameterNode { .. } => {
            optional_parameter_node::build_node(&node.as_optional_parameter_node().unwrap())
        }
        Node::OrNode { .. } => {
            or_node::build_node(&node.as_or_node().unwrap())
        }
        Node::ParametersNode { .. } => {
            parameters_node::build_node(&node.as_parameters_node().unwrap())
        }
        Node::ParenthesesNode { .. } => {
            parentheses_node::build_node(&node.as_parentheses_node().unwrap())
        }
        Node::PinnedExpressionNode { .. } => {
            pinned_expression_node::build_node(&node.as_pinned_expression_node().unwrap())
        }
        Node::PinnedVariableNode { .. } => {
            pinned_variable_node::build_node(&node.as_pinned_variable_node().unwrap())
        }
        Node::PostExecutionNode { .. } => {
            post_execution_node::build_node(&node.as_post_execution_node().unwrap())
        }
        Node::PreExecutionNode { .. } => {
            pre_execution_node::build_node(&node.as_pre_execution_node().unwrap())
        }
        Node::ProgramNode { .. } => {
            program_node::build_node(&node.as_program_node().unwrap())
        }
        Node::RangeNode { .. } => {
            range_node::build_node(&node.as_range_node().unwrap())
        }
        Node::RationalNode { .. } => {
            rational_node::build_node(&node.as_rational_node().unwrap())
        }
        Node::RedoNode { .. } => {
            redo_node::build_node(&node.as_redo_node().unwrap())
        }
        Node::RegularExpressionNode { .. } => {
            regular_expression_node::build_node(&node.as_regular_expression_node().unwrap())
        }
        Node::RequiredKeywordParameterNode { .. } => {
            required_keyword_parameter_node::build_node(&node.as_required_keyword_parameter_node().unwrap())
        }
        Node::RequiredParameterNode { .. } => {
            required_parameter_node::build_node(&node.as_required_parameter_node().unwrap())
        }
        Node::RescueModifierNode { .. } => {
            rescue_modifier_node::build_node(&node.as_rescue_modifier_node().unwrap())
        }
        Node::RescueNode { .. } => {
            rescue_node::build_node(&node.as_rescue_node().unwrap())
        }
        Node::RestParameterNode { .. } => {
            rest_parameter_node::build_node(&node.as_rest_parameter_node().unwrap())
        }
        Node::RetryNode { .. } => {
            retry_node::build_node(&node.as_retry_node().unwrap())
        }
        Node::ReturnNode { .. } => {
            return_node::build_node(&node.as_return_node().unwrap())
        }
        Node::SelfNode { .. } => {
            self_node::build_node(&node.as_self_node().unwrap())
        }
        Node::ShareableConstantNode { .. } => {
            shareable_constant_node::build_node(&node.as_shareable_constant_node().unwrap())
        }
        Node::SingletonClassNode { .. } => {
            singleton_class_node::build_node(&node.as_singleton_class_node().unwrap())
        }
        Node::SourceEncodingNode { .. } => {
            source_encoding_node::build_node(&node.as_source_encoding_node().unwrap())
        }
        Node::SourceFileNode { .. } => {
            source_file_node::build_node(&node.as_source_file_node().unwrap())
        }
        Node::SourceLineNode { .. } => {
            source_line_node::build_node(&node.as_source_line_node().unwrap())
        }
        Node::SplatNode { .. } => {
            splat_node::build_node(&node.as_splat_node().unwrap())
        }
        Node::StatementsNode { .. } => {
            statements_node::build_node(&node.as_statements_node().unwrap())
        }
        Node::StringNode { .. } => {
            string_node::build_node(&node.as_string_node().unwrap())
        }
        Node::SuperNode { .. } => {
            super_node::build_node(&node.as_super_node().unwrap())
        }
        Node::SymbolNode { .. } => {
            symbol_node::build_node(&node.as_symbol_node().unwrap())
        }
        Node::TrueNode { .. } => {
            true_node::build_node(&node.as_true_node().unwrap())
        }
        Node::UndefNode { .. } => {
            undef_node::build_node(&node.as_undef_node().unwrap())
        }
        Node::UnlessNode { .. } => {
            unless_node::build_node(&node.as_unless_node().unwrap())
        }
        Node::UntilNode { .. } => {
            until_node::build_node(&node.as_until_node().unwrap())
        }
        Node::WhenNode { .. } => {
            when_node::build_node(&node.as_when_node().unwrap())
        }
        Node::WhileNode { .. } => {
            while_node::build_node(&node.as_while_node().unwrap())
        }
        Node::XStringNode { .. } => {
            x_string_node::build_node(&node.as_x_string_node().unwrap())
        }
        Node::YieldNode { .. } => {
            yield_node::build_node(&node.as_yield_node().unwrap())
        }
    }
}
