use crate::builder::node::*;
use crate::doc::*;
use ruby_prism::*;

#[rustfmt::skip]
pub fn build(node: &Node) -> Doc {
    match node {
        Node::AliasGlobalVariableNode { .. } => {
            println_dbg!("AliasGlobalVariableNode");
            alias_global_variable_node::build_node(&node.as_alias_global_variable_node().unwrap())
        }
        Node::AliasMethodNode { .. } => {
            println_dbg!("AliasMethodNode");
            alias_method_node::build_node(&node.as_alias_method_node().unwrap())
        }
        Node::AlternationPatternNode { .. } => {
            println_dbg!("AlternationPatternNode");
            alternation_pattern_node::build_node(&node.as_alternation_pattern_node().unwrap())
        }
        Node::AndNode { .. } => {
            println_dbg!("AndNode");
            and_node::build_node(&node.as_and_node().unwrap())
        }
        Node::ArgumentsNode { .. } => {
            println_dbg!("ArgumentsNode");
            arguments_node::build_node(&node.as_arguments_node().unwrap())
        }
        Node::ArrayNode { .. } => {
            println_dbg!("ArrayNode");
            array_node::build_node(&node.as_array_node().unwrap())
        }
        Node::ArrayPatternNode { .. } => {
            println_dbg!("ArrayPatternNode");
            array_pattern_node::build_node(&node.as_array_pattern_node().unwrap())
        }
        Node::AssocNode { .. } => {
            println_dbg!("AssocNode");
            assoc_node::build_node(&node.as_assoc_node().unwrap())
        }
        Node::AssocSplatNode { .. } => {
            println_dbg!("AssocSplatNode");
            assoc_splat_node::build_node(&node.as_assoc_splat_node().unwrap())
        }
        Node::BackReferenceReadNode { .. } => {
            println_dbg!("BackReferenceReadNode");
            back_reference_read_node::build_node(&node.as_back_reference_read_node().unwrap())
        }
        Node::BeginNode { .. } => {
            println_dbg!("BeginNode");
            begin_node::build_node(&node.as_begin_node().unwrap())
        }
        Node::BlockArgumentNode { .. } => {
            println_dbg!("BlockArgumentNode");
            block_argument_node::build_node(&node.as_block_argument_node().unwrap())
        }
        Node::BlockLocalVariableNode { .. } => {
            println_dbg!("BlockLocalVariableNode");
            block_local_variable_node::build_node(&node.as_block_local_variable_node().unwrap())
        }
        Node::BlockNode { .. } => {
            println_dbg!("BlockNode");
            block_node::build_node(&node.as_block_node().unwrap())
        }
        Node::BlockParameterNode { .. } => {
            println_dbg!("BlockParameterNode");
            block_parameter_node::build_node(&node.as_block_parameter_node().unwrap())
        }
        Node::BlockParametersNode { .. } => {
            println_dbg!("BlockParametersNode");
            block_parameters_node::build_node(&node.as_block_parameters_node().unwrap())
        }
        Node::BreakNode { .. } => {
            println_dbg!("BreakNode");
            break_node::build_node(&node.as_break_node().unwrap())
        }
        Node::CallAndWriteNode { .. } => {
            println_dbg!("CallAndWriteNode");
            call_and_write_node::build_node(&node.as_call_and_write_node().unwrap())
        }
        Node::CallNode { .. } => {
            println_dbg!("CallNode");
            call_node::build_node(&node.as_call_node().unwrap())
        }
        Node::CallOperatorWriteNode { .. } => {
            println_dbg!("CallOperatorWriteNode");
            call_operator_write_node::build_node(&node.as_call_operator_write_node().unwrap())
        }
        Node::CallOrWriteNode { .. } => {
            println_dbg!("CallOrWriteNode");
            call_or_write_node::build_node(&node.as_call_or_write_node().unwrap())
        }
        Node::CallTargetNode { .. } => {
            println_dbg!("CallTargetNode");
            call_target_node::build_node(&node.as_call_target_node().unwrap())
        }
        Node::CapturePatternNode { .. } => {
            println_dbg!("CapturePatternNode");
            capture_pattern_node::build_node(&node.as_capture_pattern_node().unwrap())
        }
        Node::CaseMatchNode { .. } => {
            println_dbg!("CaseMatchNode");
            case_match_node::build_node(&node.as_case_match_node().unwrap())
        }
        Node::CaseNode { .. } => {
            println_dbg!("CaseNode");
            case_node::build_node(&node.as_case_node().unwrap())
        }
        Node::ClassNode { .. } => {
            println_dbg!("ClassNode");
            class_node::build_node(&node.as_class_node().unwrap())
        }
        Node::ClassVariableAndWriteNode { .. } => {
            println_dbg!("ClassVariableAndWriteNode");
            class_variable_and_write_node::build_node(&node.as_class_variable_and_write_node().unwrap())
        }
        Node::ClassVariableOperatorWriteNode { .. } => {
            println_dbg!("ClassVariableOperatorWriteNode");
            class_variable_operator_write_node::build_node(&node.as_class_variable_operator_write_node().unwrap())
        }
        Node::ClassVariableOrWriteNode { .. } => {
            println_dbg!("ClassVariableOrWriteNode");
            class_variable_or_write_node::build_node(&node.as_class_variable_or_write_node().unwrap())
        }
        Node::ClassVariableReadNode { .. } => {
            println_dbg!("ClassVariableReadNode");
            class_variable_read_node::build_node(&node.as_class_variable_read_node().unwrap())
        }
        Node::ClassVariableTargetNode { .. } => {
            println_dbg!("ClassVariableTargetNode");
            class_variable_target_node::build_node(&node.as_class_variable_target_node().unwrap())
        }
        Node::ClassVariableWriteNode { .. } => {
            println_dbg!("ClassVariableWriteNode");
            class_variable_write_node::build_node(&node.as_class_variable_write_node().unwrap())
        }
        Node::ConstantAndWriteNode { .. } => {
            println_dbg!("ConstantAndWriteNode");
            constant_and_write_node::build_node(&node.as_constant_and_write_node().unwrap())
        }
        Node::ConstantOperatorWriteNode { .. } => {
            println_dbg!("ConstantOperatorWriteNode");
            constant_operator_write_node::build_node(&node.as_constant_operator_write_node().unwrap())
        }
        Node::ConstantOrWriteNode { .. } => {
            println_dbg!("ConstantOrWriteNode");
            constant_or_write_node::build_node(&node.as_constant_or_write_node().unwrap())
        }
        Node::ConstantPathAndWriteNode { .. } => {
            println_dbg!("ConstantPathAndWriteNode");
            constant_path_and_write_node::build_node(&node.as_constant_path_and_write_node().unwrap())
        }
        Node::ConstantPathNode { .. } => {
            println_dbg!("ConstantPathNode");
            constant_path_node::build_node(&node.as_constant_path_node().unwrap())
        }
        Node::ConstantPathOperatorWriteNode { .. } => {
            println_dbg!("ConstantPathOperatorWriteNode");
            constant_path_operator_write_node::build_node(&node.as_constant_path_operator_write_node().unwrap())
        }
        Node::ConstantPathOrWriteNode { .. } => {
            println_dbg!("ConstantPathOrWriteNode");
            constant_path_or_write_node::build_node(&node.as_constant_path_or_write_node().unwrap())
        }
        Node::ConstantPathTargetNode { .. } => {
            println_dbg!("ConstantPathTargetNode");
            constant_path_target_node::build_node(&node.as_constant_path_target_node().unwrap())
        }
        Node::ConstantPathWriteNode { .. } => {
            println_dbg!("ConstantPathWriteNode");
            constant_path_write_node::build_node(&node.as_constant_path_write_node().unwrap())
        }
        Node::ConstantReadNode { .. } => {
            println_dbg!("ConstantReadNode");
            constant_read_node::build_node(&node.as_constant_read_node().unwrap())
        }
        Node::ConstantTargetNode { .. } => {
            println_dbg!("ConstantTargetNode");
            constant_target_node::build_node(&node.as_constant_target_node().unwrap())
        }
        Node::ConstantWriteNode { .. } => {
            println_dbg!("ConstantWriteNode");
            constant_write_node::build_node(&node.as_constant_write_node().unwrap())
        }
        Node::DefNode { .. } => {
            println_dbg!("DefNode");
            def_node::build_node(&node.as_def_node().unwrap())
        }
        Node::DefinedNode { .. } => {
            println_dbg!("DefinedNode");
            defined_node::build_node(&node.as_defined_node().unwrap())
        }
        Node::ElseNode { .. } => {
            println_dbg!("ElseNode");
            else_node::build_node(&node.as_else_node().unwrap())
        }
        Node::EmbeddedStatementsNode { .. } => {
            println_dbg!("EmbeddedStatementsNode");
            embedded_statements_node::build_node(&node.as_embedded_statements_node().unwrap())
        }
        Node::EmbeddedVariableNode { .. } => {
            println_dbg!("EmbeddedVariableNode");
            embedded_variable_node::build_node(&node.as_embedded_variable_node().unwrap())
        }
        Node::EnsureNode { .. } => {
            println_dbg!("EnsureNode");
            ensure_node::build_node(&node.as_ensure_node().unwrap())
        }
        Node::FalseNode { .. } => {
            println_dbg!("FalseNode");
            false_node::build_node(&node.as_false_node().unwrap())
        }
        Node::FindPatternNode { .. } => {
            println_dbg!("FindPatternNode");
            find_pattern_node::build_node(&node.as_find_pattern_node().unwrap())
        }
        Node::FlipFlopNode { .. } => {
            println_dbg!("FlipFlopNode");
            flip_flop_node::build_node(&node.as_flip_flop_node().unwrap())
        }
        Node::FloatNode { .. } => {
            println_dbg!("FloatNode");
            float_node::build_node(&node.as_float_node().unwrap())
        }
        Node::ForNode { .. } => {
            println_dbg!("ForNode");
            for_node::build_node(&node.as_for_node().unwrap())
        }
        Node::ForwardingArgumentsNode { .. } => {
            println_dbg!("ForwardingArgumentsNode");
            forwarding_arguments_node::build_node(&node.as_forwarding_arguments_node().unwrap())
        }
        Node::ForwardingParameterNode { .. } => {
            println_dbg!("ForwardingParameterNode");
            forwarding_parameter_node::build_node(&node.as_forwarding_parameter_node().unwrap())
        }
        Node::ForwardingSuperNode { .. } => {
            println_dbg!("ForwardingSuperNode");
            forwarding_super_node::build_node(&node.as_forwarding_super_node().unwrap())
        }
        Node::GlobalVariableAndWriteNode { .. } => {
            println_dbg!("GlobalVariableAndWriteNode");
            global_variable_and_write_node::build_node(&node.as_global_variable_and_write_node().unwrap())
        }
        Node::GlobalVariableOperatorWriteNode { .. } => {
            println_dbg!("GlobalVariableOperatorWriteNode");
            global_variable_operator_write_node::build_node(&node.as_global_variable_operator_write_node().unwrap())
        }
        Node::GlobalVariableOrWriteNode { .. } => {
            println_dbg!("GlobalVariableOrWriteNode");
            global_variable_or_write_node::build_node(&node.as_global_variable_or_write_node().unwrap())
        }
        Node::GlobalVariableReadNode { .. } => {
            println_dbg!("GlobalVariableReadNode");
            global_variable_read_node::build_node(&node.as_global_variable_read_node().unwrap())
        }
        Node::GlobalVariableTargetNode { .. } => {
            println_dbg!("GlobalVariableTargetNode");
            global_variable_target_node::build_node(&node.as_global_variable_target_node().unwrap())
        }
        Node::GlobalVariableWriteNode { .. } => {
            println_dbg!("GlobalVariableWriteNode");
            global_variable_write_node::build_node(&node.as_global_variable_write_node().unwrap())
        }
        Node::HashNode { .. } => {
            println_dbg!("HashNode");
            hash_node::build_node(&node.as_hash_node().unwrap())
        }
        Node::HashPatternNode { .. } => {
            println_dbg!("HashPatternNode");
            hash_pattern_node::build_node(&node.as_hash_pattern_node().unwrap())
        }
        Node::IfNode { .. } => {
            println_dbg!("IfNode");
            if_node::build_node(&node.as_if_node().unwrap())
        }
        Node::ImaginaryNode { .. } => {
            println_dbg!("ImaginaryNode");
            imaginary_node::build_node(&node.as_imaginary_node().unwrap())
        }
        Node::ImplicitNode { .. } => {
            println_dbg!("ImplicitNode");
            implicit_node::build_node(&node.as_implicit_node().unwrap())
        }
        Node::ImplicitRestNode { .. } => {
            println_dbg!("ImplicitRestNode");
            implicit_rest_node::build_node(&node.as_implicit_rest_node().unwrap())
        }
        Node::InNode { .. } => {
            println_dbg!("InNode");
            in_node::build_node(&node.as_in_node().unwrap())
        }
        Node::IndexAndWriteNode { .. } => {
            println_dbg!("IndexAndWriteNode");
            index_and_write_node::build_node(&node.as_index_and_write_node().unwrap())
        }
        Node::IndexOperatorWriteNode { .. } => {
            println_dbg!("IndexOperatorWriteNode");
            index_operator_write_node::build_node(&node.as_index_operator_write_node().unwrap())
        }
        Node::IndexOrWriteNode { .. } => {
            println_dbg!("IndexOrWriteNode");
            index_or_write_node::build_node(&node.as_index_or_write_node().unwrap())
        }
        Node::IndexTargetNode { .. } => {
            println_dbg!("IndexTargetNode");
            index_target_node::build_node(&node.as_index_target_node().unwrap())
        }
        Node::InstanceVariableAndWriteNode { .. } => {
            println_dbg!("InstanceVariableAndWriteNode");
            instance_variable_and_write_node::build_node(&node.as_instance_variable_and_write_node().unwrap())
        }
        Node::InstanceVariableOperatorWriteNode { .. } => {
            println_dbg!("InstanceVariableOperatorWriteNode");
            instance_variable_operator_write_node::build_node(&node.as_instance_variable_operator_write_node().unwrap())
        }
        Node::InstanceVariableOrWriteNode { .. } => {
            println_dbg!("InstanceVariableOrWriteNode");
            instance_variable_or_write_node::build_node(&node.as_instance_variable_or_write_node().unwrap())
        }
        Node::InstanceVariableReadNode { .. } => {
            println_dbg!("InstanceVariableReadNode");
            instance_variable_read_node::build_node(&node.as_instance_variable_read_node().unwrap())
        }
        Node::InstanceVariableTargetNode { .. } => {
            println_dbg!("InstanceVariableTargetNode");
            instance_variable_target_node::build_node(&node.as_instance_variable_target_node().unwrap())
        }
        Node::InstanceVariableWriteNode { .. } => {
            println_dbg!("InstanceVariableWriteNode");
            instance_variable_write_node::build_node(&node.as_instance_variable_write_node().unwrap())
        }
        Node::IntegerNode { .. } => {
            println_dbg!("IntegerNode");
            integer_node::build_node(&node.as_integer_node().unwrap())
        }
        Node::InterpolatedMatchLastLineNode { .. } => {
            println_dbg!("InterpolatedMatchLastLineNode");
            interpolated_match_last_line_node::build_node(&node.as_interpolated_match_last_line_node().unwrap())
        }
        Node::InterpolatedRegularExpressionNode { .. } => {
            println_dbg!("InterpolatedRegularExpressionNode");
            interpolated_regular_expression_node::build_node(&node.as_interpolated_regular_expression_node().unwrap())
        }
        Node::InterpolatedStringNode { .. } => {
            println_dbg!("InterpolatedStringNode");
            interpolated_string_node::build_node(&node.as_interpolated_string_node().unwrap())
        }
        Node::InterpolatedSymbolNode { .. } => {
            println_dbg!("InterpolatedSymbolNode");
            interpolated_symbol_node::build_node(&node.as_interpolated_symbol_node().unwrap())
        }
        Node::InterpolatedXStringNode { .. } => {
            println_dbg!("InterpolatedXStringNode");
            interpolated_x_string_node::build_node(&node.as_interpolated_x_string_node().unwrap())
        }
        Node::ItLocalVariableReadNode { .. } => {
            println_dbg!("ItLocalVariableReadNode");
            it_local_variable_read_node::build_node(&node.as_it_local_variable_read_node().unwrap())
        }
        Node::ItParametersNode { .. } => {
            println_dbg!("ItParametersNode");
            it_parameters_node::build_node(&node.as_it_parameters_node().unwrap())
        }
        Node::KeywordHashNode { .. } => {
            println_dbg!("KeywordHashNode");
            keyword_hash_node::build_node(&node.as_keyword_hash_node().unwrap())
        }
        Node::KeywordRestParameterNode { .. } => {
            println_dbg!("KeywordRestParameterNode");
            keyword_rest_parameter_node::build_node(&node.as_keyword_rest_parameter_node().unwrap())
        }
        Node::LambdaNode { .. } => {
            println_dbg!("LambdaNode");
            lambda_node::build_node(&node.as_lambda_node().unwrap())
        }
        Node::LocalVariableAndWriteNode { .. } => {
            println_dbg!("LocalVariableAndWriteNode");
            local_variable_and_write_node::build_node(&node.as_local_variable_and_write_node().unwrap())
        }
        Node::LocalVariableOperatorWriteNode { .. } => {
            println_dbg!("LocalVariableOperatorWriteNode");
            local_variable_operator_write_node::build_node(&node.as_local_variable_operator_write_node().unwrap())
        }
        Node::LocalVariableOrWriteNode { .. } => {
            println_dbg!("LocalVariableOrWriteNode");
            local_variable_or_write_node::build_node(&node.as_local_variable_or_write_node().unwrap())
        }
        Node::LocalVariableReadNode { .. } => {
            println_dbg!("LocalVariableReadNode");
            local_variable_read_node::build_node(&node.as_local_variable_read_node().unwrap())
        }
        Node::LocalVariableTargetNode { .. } => {
            println_dbg!("LocalVariableTargetNode");
            local_variable_target_node::build_node(&node.as_local_variable_target_node().unwrap())
        }
        Node::LocalVariableWriteNode { .. } => {
            println_dbg!("LocalVariableWriteNode");
            local_variable_write_node::build_node(&node.as_local_variable_write_node().unwrap())
        }
        Node::MatchLastLineNode { .. } => {
            println_dbg!("MatchLastLineNode");
            match_last_line_node::build_node(&node.as_match_last_line_node().unwrap())
        }
        Node::MatchPredicateNode { .. } => {
            println_dbg!("MatchPredicateNode");
            match_predicate_node::build_node(&node.as_match_predicate_node().unwrap())
        }
        Node::MatchRequiredNode { .. } => {
            println_dbg!("MatchRequiredNode");
            match_required_node::build_node(&node.as_match_required_node().unwrap())
        }
        Node::MatchWriteNode { .. } => {
            println_dbg!("MatchWriteNode");
            match_write_node::build_node(&node.as_match_write_node().unwrap())
        }
        Node::MissingNode { .. } => {
            println_dbg!("MissingNode");
            missing_node::build_node(&node.as_missing_node().unwrap())
        }
        Node::ModuleNode { .. } => {
            println_dbg!("ModuleNode");
            module_node::build_node(&node.as_module_node().unwrap())
        }
        Node::MultiTargetNode { .. } => {
            println_dbg!("MultiTargetNode");
            multi_target_node::build_node(&node.as_multi_target_node().unwrap())
        }
        Node::MultiWriteNode { .. } => {
            println_dbg!("MultiWriteNode");
            multi_write_node::build_node(&node.as_multi_write_node().unwrap())
        }
        Node::NextNode { .. } => {
            println_dbg!("NextNode");
            next_node::build_node(&node.as_next_node().unwrap())
        }
        Node::NilNode { .. } => {
            println_dbg!("NilNode");
            nil_node::build_node(&node.as_nil_node().unwrap())
        }
        Node::NoKeywordsParameterNode { .. } => {
            println_dbg!("NoKeywordsParameterNode");
            no_keywords_parameter_node::build_node(&node.as_no_keywords_parameter_node().unwrap())
        }
        Node::NumberedParametersNode { .. } => {
            println_dbg!("NumberedParametersNode");
            numbered_parameters_node::build_node(&node.as_numbered_parameters_node().unwrap())
        }
        Node::NumberedReferenceReadNode { .. } => {
            println_dbg!("NumberedReferenceReadNode");
            numbered_reference_read_node::build_node(&node.as_numbered_reference_read_node().unwrap())
        }
        Node::OptionalKeywordParameterNode { .. } => {
            println_dbg!("OptionalKeywordParameterNode");
            optional_keyword_parameter_node::build_node(&node.as_optional_keyword_parameter_node().unwrap())
        }
        Node::OptionalParameterNode { .. } => {
            println_dbg!("OptionalParameterNode");
            optional_parameter_node::build_node(&node.as_optional_parameter_node().unwrap())
        }
        Node::OrNode { .. } => {
            println_dbg!("OrNode");
            or_node::build_node(&node.as_or_node().unwrap())
        }
        Node::ParametersNode { .. } => {
            println_dbg!("ParametersNode");
            parameters_node::build_node(&node.as_parameters_node().unwrap())
        }
        Node::ParenthesesNode { .. } => {
            println_dbg!("ParenthesesNode");
            parentheses_node::build_node(&node.as_parentheses_node().unwrap())
        }
        Node::PinnedExpressionNode { .. } => {
            println_dbg!("PinnedExpressionNode");
            pinned_expression_node::build_node(&node.as_pinned_expression_node().unwrap())
        }
        Node::PinnedVariableNode { .. } => {
            println_dbg!("PinnedVariableNode");
            pinned_variable_node::build_node(&node.as_pinned_variable_node().unwrap())
        }
        Node::PostExecutionNode { .. } => {
            println_dbg!("PostExecutionNode");
            post_execution_node::build_node(&node.as_post_execution_node().unwrap())
        }
        Node::PreExecutionNode { .. } => {
            println_dbg!("PreExecutionNode");
            pre_execution_node::build_node(&node.as_pre_execution_node().unwrap())
        }
        Node::ProgramNode { .. } => {
            println_dbg!("ProgramNode");
            program_node::build_node(&node.as_program_node().unwrap())
        }
        Node::RangeNode { .. } => {
            println_dbg!("RangeNode");
            range_node::build_node(&node.as_range_node().unwrap())
        }
        Node::RationalNode { .. } => {
            println_dbg!("RationalNode");
            rational_node::build_node(&node.as_rational_node().unwrap())
        }
        Node::RedoNode { .. } => {
            println_dbg!("RedoNode");
            redo_node::build_node(&node.as_redo_node().unwrap())
        }
        Node::RegularExpressionNode { .. } => {
            println_dbg!("RegularExpressionNode");
            regular_expression_node::build_node(&node.as_regular_expression_node().unwrap())
        }
        Node::RequiredKeywordParameterNode { .. } => {
            println_dbg!("RequiredKeywordParameterNode");
            required_keyword_parameter_node::build_node(&node.as_required_keyword_parameter_node().unwrap())
        }
        Node::RequiredParameterNode { .. } => {
            println_dbg!("RequiredParameterNode");
            required_parameter_node::build_node(&node.as_required_parameter_node().unwrap())
        }
        Node::RescueModifierNode { .. } => {
            println_dbg!("RescueModifierNode");
            rescue_modifier_node::build_node(&node.as_rescue_modifier_node().unwrap())
        }
        Node::RescueNode { .. } => {
            println_dbg!("RescueNode");
            rescue_node::build_node(&node.as_rescue_node().unwrap())
        }
        Node::RestParameterNode { .. } => {
            println_dbg!("RestParameterNode");
            rest_parameter_node::build_node(&node.as_rest_parameter_node().unwrap())
        }
        Node::RetryNode { .. } => {
            println_dbg!("RetryNode");
            retry_node::build_node(&node.as_retry_node().unwrap())
        }
        Node::ReturnNode { .. } => {
            println_dbg!("ReturnNode");
            return_node::build_node(&node.as_return_node().unwrap())
        }
        Node::SelfNode { .. } => {
            println_dbg!("SelfNode");
            self_node::build_node(&node.as_self_node().unwrap())
        }
        Node::ShareableConstantNode { .. } => {
            println_dbg!("ShareableConstantNode");
            shareable_constant_node::build_node(&node.as_shareable_constant_node().unwrap())
        }
        Node::SingletonClassNode { .. } => {
            println_dbg!("SingletonClassNode");
            singleton_class_node::build_node(&node.as_singleton_class_node().unwrap())
        }
        Node::SourceEncodingNode { .. } => {
            println_dbg!("SourceEncodingNode");
            source_encoding_node::build_node(&node.as_source_encoding_node().unwrap())
        }
        Node::SourceFileNode { .. } => {
            println_dbg!("SourceFileNode");
            source_file_node::build_node(&node.as_source_file_node().unwrap())
        }
        Node::SourceLineNode { .. } => {
            println_dbg!("SourceLineNode");
            source_line_node::build_node(&node.as_source_line_node().unwrap())
        }
        Node::SplatNode { .. } => {
            println_dbg!("SplatNode");
            splat_node::build_node(&node.as_splat_node().unwrap())
        }
        Node::StatementsNode { .. } => {
            println_dbg!("StatementsNode");
            statements_node::build_node(&node.as_statements_node().unwrap())
        }
        Node::StringNode { .. } => {
            println_dbg!("StringNode");
            string_node::build_node(&node.as_string_node().unwrap())
        }
        Node::SuperNode { .. } => {
            println_dbg!("SuperNode");
            super_node::build_node(&node.as_super_node().unwrap())
        }
        Node::SymbolNode { .. } => {
            println_dbg!("SymbolNode");
            symbol_node::build_node(&node.as_symbol_node().unwrap())
        }
        Node::TrueNode { .. } => {
            println_dbg!("TrueNode");
            true_node::build_node(&node.as_true_node().unwrap())
        }
        Node::UndefNode { .. } => {
            println_dbg!("UndefNode");
            undef_node::build_node(&node.as_undef_node().unwrap())
        }
        Node::UnlessNode { .. } => {
            println_dbg!("UnlessNode");
            unless_node::build_node(&node.as_unless_node().unwrap())
        }
        Node::UntilNode { .. } => {
            println_dbg!("UntilNode");
            until_node::build_node(&node.as_until_node().unwrap())
        }
        Node::WhenNode { .. } => {
            println_dbg!("WhenNode");
            when_node::build_node(&node.as_when_node().unwrap())
        }
        Node::WhileNode { .. } => {
            println_dbg!("WhileNode");
            while_node::build_node(&node.as_while_node().unwrap())
        }
        Node::XStringNode { .. } => {
            println_dbg!("XStringNode");
            x_string_node::build_node(&node.as_x_string_node().unwrap())
        }
        Node::YieldNode { .. } => {
            println_dbg!("YieldNode");
            yield_node::build_node(&node.as_yield_node().unwrap())
        }
    }
}
