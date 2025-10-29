use crate::builder::builder::hardline;
use crate::builder::prism_node::node::*;
use crate::document::Document;
use ruby_prism::{Comments, Node};
use std::{io::Read, iter::Peekable};

pub struct BuildContext<'a> {
    pub source: &'a [u8],
    pub built_end: usize,
    pub comments: &'a mut Peekable<Comments<'a>>,
    pub is_statement: bool,
}

pub trait BuildPrismNode {
    //
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document;
    //
    #[rustfmt::skip]
    fn build(&self, context: &mut BuildContext) -> Document {
        self._build(context)
    }
    //
    #[rustfmt::skip]
    fn build_with(
        &self, context: &mut BuildContext, before: Option<Document>, after: Option<Document>) -> Document {
        let before = before.unwrap_or(Document::None);
        let after = after.unwrap_or(Document::None);
        Document::Array(Vec::from([before, self.build(context), after]))
    }
}

fn build_leading_line_breaks(
    context: &mut BuildContext,
    start_offset: usize,
    max_line_breaks: usize,
) -> Option<Document> {
    fn is_indent_char(c: &u8) -> bool {
        matches!(c, b' ' | b'\t')
    }
    fn is_line_break_char(c: &u8) -> bool {
        matches!(c, b'\n')
    }
    let gap_start = context.built_end;
    let gap_end = start_offset;
    if gap_start < gap_end {
        let mut i = gap_start;
        let mut break_lines = 0usize;
        while i < gap_end {
            if is_line_break_char(&context.source[i]) {
                let mut j = i + 1;
                while j < gap_end && is_indent_char(&context.source[j]) {
                    j += 1;
                }
                if j < gap_end && is_line_break_char(&context.source[j]) {
                    break_lines += 1;
                    if break_lines >= max_line_breaks {
                        break;
                    }
                    i = j;
                    continue;
                }
                i = j;
            }
            i += 1;
        }
        if break_lines > 0 {
            context.built_end = gap_end.min(context.source.len());
            return Some(Document::Array(vec![hardline(); break_lines]));
        }
    }
    None
}

impl BuildPrismNode for Node<'_> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        let mut vec = Vec::new();


        if context.is_statement {
            if let Some(breaks) = build_leading_line_breaks(context, self.location().start_offset(), 1usize) {
                vec.push(breaks);
            }
        }
        let prev_is_statement = context.is_statement;
        context.is_statement = match self {
            Node::StatementsNode { .. } => true,
            Node::ProgramNode { .. } => true,
            _ => false,
        };


        loop {
            let next_comment_start = context.comments.peek().map(|c| c.location().start_offset());
            match next_comment_start {
                Some(start) => {
                    // Leading comments
                    if start < self.location().start_offset() {
                        if let Some(breaks) = build_leading_line_breaks(context, start, 2usize) {
                            vec.push(breaks);
                        }
                        let comment = context.comments.next().unwrap();
                        let mut buf = String::new();
                        comment.text().read_to_string(&mut buf).unwrap();
                        vec.push(Document::String(buf));
                        vec.push(hardline());
                    } else {
                        break;
                    }
                }
                None => break,
            }

        }



        let built_node = match self {
            Node::AliasGlobalVariableNode { .. } => {
                alias_global_variable_node::build_node(self.as_alias_global_variable_node().as_ref(), context)
            }
            Node::AliasMethodNode { .. } => {
                alias_method_node::build_node(self.as_alias_method_node().as_ref(), context)
            }
            Node::AlternationPatternNode { .. } => {
                alternation_pattern_node::build_node(self.as_alternation_pattern_node().as_ref(), context)
            }
            Node::AndNode { .. } => {
                and_node::build_node(self.as_and_node().as_ref(), context)
            }
            Node::ArgumentsNode { .. } => {
                arguments_node::build_node(self.as_arguments_node().as_ref(), context)
            }
            Node::ArrayNode { .. } => {
                array_node::build_node(self.as_array_node().as_ref(), context)
            }
            Node::ArrayPatternNode { .. } => {
                array_pattern_node::build_node(self.as_array_pattern_node().as_ref(), context)
            }
            Node::AssocNode { .. } => {
                assoc_node::build_node(self.as_assoc_node().as_ref(), context)
            }
            Node::AssocSplatNode { .. } => {
                assoc_splat_node::build_node(self.as_assoc_splat_node().as_ref(), context)
            }
            Node::BackReferenceReadNode { .. } => {
                back_reference_read_node::build_node(self.as_back_reference_read_node().as_ref(), context)
            }
            Node::BeginNode { .. } => {
                begin_node::build_node(self.as_begin_node().as_ref(), context)
            }
            Node::BlockArgumentNode { .. } => {
                block_argument_node::build_node(self.as_block_argument_node().as_ref(), context)
            }
            Node::BlockLocalVariableNode { .. } => {
                block_local_variable_node::build_node(self.as_block_local_variable_node().as_ref(), context)
            }
            Node::BlockNode { .. } => {
                block_node::build_node(self.as_block_node().as_ref(), context)
            }
            Node::BlockParameterNode { .. } => {
                block_parameter_node::build_node(self.as_block_parameter_node().as_ref(), context)
            }
            Node::BlockParametersNode { .. } => {
                block_parameters_node::build_node(self.as_block_parameters_node().as_ref(), context)
            }
            Node::BreakNode { .. } => {
                break_node::build_node(self.as_break_node().as_ref(), context)
            }
            Node::CallAndWriteNode { .. } => {
                call_and_write_node::build_node(self.as_call_and_write_node().as_ref(), context)
            }
            Node::CallNode { .. } => {
                call_node::build_node(self.as_call_node().as_ref(), context)
            }
            Node::CallOperatorWriteNode { .. } => {
                call_operator_write_node::build_node(self.as_call_operator_write_node().as_ref(), context)
            }
            Node::CallOrWriteNode { .. } => {
                call_or_write_node::build_node(self.as_call_or_write_node().as_ref(), context)
            }
            Node::CallTargetNode { .. } => {
                call_target_node::build_node(self.as_call_target_node().as_ref(), context)
            }
            Node::CapturePatternNode { .. } => {
                capture_pattern_node::build_node(self.as_capture_pattern_node().as_ref(), context)
            }
            Node::CaseMatchNode { .. } => {
                case_match_node::build_node(self.as_case_match_node().as_ref(), context)
            }
            Node::CaseNode { .. } => {
                case_node::build_node(self.as_case_node().as_ref(), context)
            }
            Node::ClassNode { .. } => {
                class_node::build_node(self.as_class_node().as_ref(), context)
            }
            Node::ClassVariableAndWriteNode { .. } => {
                class_variable_and_write_node::build_node(self.as_class_variable_and_write_node().as_ref(), context)
            }
            Node::ClassVariableOperatorWriteNode { .. } => {
                class_variable_operator_write_node::build_node(self.as_class_variable_operator_write_node().as_ref(), context)
            }
            Node::ClassVariableOrWriteNode { .. } => {
                class_variable_or_write_node::build_node(self.as_class_variable_or_write_node().as_ref(), context)
            }
            Node::ClassVariableReadNode { .. } => {
                class_variable_read_node::build_node(self.as_class_variable_read_node().as_ref(), context)
            }
            Node::ClassVariableTargetNode { .. } => {
                class_variable_target_node::build_node(self.as_class_variable_target_node().as_ref(), context)
            }
            Node::ClassVariableWriteNode { .. } => {
                class_variable_write_node::build_node(self.as_class_variable_write_node().as_ref(), context)
            }
            Node::ConstantAndWriteNode { .. } => {
                constant_and_write_node::build_node(self.as_constant_and_write_node().as_ref(), context)
            }
            Node::ConstantOperatorWriteNode { .. } => {
                constant_operator_write_node::build_node(self.as_constant_operator_write_node().as_ref(), context)
            }
            Node::ConstantOrWriteNode { .. } => {
                constant_or_write_node::build_node(self.as_constant_or_write_node().as_ref(), context)
            }
            Node::ConstantPathAndWriteNode { .. } => {
                constant_path_and_write_node::build_node(self.as_constant_path_and_write_node().as_ref(), context)
            }
            Node::ConstantPathNode { .. } => {
                constant_path_node::build_node(self.as_constant_path_node().as_ref(), context)
            }
            Node::ConstantPathOperatorWriteNode { .. } => {
                constant_path_operator_write_node::build_node(self.as_constant_path_operator_write_node().as_ref(), context)
            }
            Node::ConstantPathOrWriteNode { .. } => {
                constant_path_or_write_node::build_node(self.as_constant_path_or_write_node().as_ref(), context)
            }
            Node::ConstantPathTargetNode { .. } => {
                constant_path_target_node::build_node(self.as_constant_path_target_node().as_ref(), context)
            }
            Node::ConstantPathWriteNode { .. } => {
                constant_path_write_node::build_node(self.as_constant_path_write_node().as_ref(), context)
            }
            Node::ConstantReadNode { .. } => {
                constant_read_node::build_node(self.as_constant_read_node().as_ref(), context)
            }
            Node::ConstantTargetNode { .. } => {
                constant_target_node::build_node(self.as_constant_target_node().as_ref(), context)
            }
            Node::ConstantWriteNode { .. } => {
                constant_write_node::build_node(self.as_constant_write_node().as_ref(), context)
            }
            Node::DefNode { .. } => {
                def_node::build_node(self.as_def_node().as_ref(), context)
            }
            Node::DefinedNode { .. } => {
                defined_node::build_node(self.as_defined_node().as_ref(), context)
            }
            Node::ElseNode { .. } => {
                else_node::build_node(self.as_else_node().as_ref(), context)
            }
            Node::EmbeddedStatementsNode { .. } => {
                embedded_statements_node::build_node(self.as_embedded_statements_node().as_ref(), context)
            }
            Node::EmbeddedVariableNode { .. } => {
                embedded_variable_node::build_node(self.as_embedded_variable_node().as_ref(), context)
            }
            Node::EnsureNode { .. } => {
                ensure_node::build_node(self.as_ensure_node().as_ref(), context)
            }
            Node::FalseNode { .. } => {
                false_node::build_node(self.as_false_node().as_ref(), context)
            }
            Node::FindPatternNode { .. } => {
                find_pattern_node::build_node(self.as_find_pattern_node().as_ref(), context)
            }
            Node::FlipFlopNode { .. } => {
                flip_flop_node::build_node(self.as_flip_flop_node().as_ref(), context)
            }
            Node::FloatNode { .. } => {
                float_node::build_node(self.as_float_node().as_ref(), context)
            }
            Node::ForNode { .. } => {
                for_node::build_node(self.as_for_node().as_ref(), context)
            }
            Node::ForwardingArgumentsNode { .. } => {
                forwarding_arguments_node::build_node(self.as_forwarding_arguments_node().as_ref(), context)
            }
            Node::ForwardingParameterNode { .. } => {
                forwarding_parameter_node::build_node(self.as_forwarding_parameter_node().as_ref(), context)
            }
            Node::ForwardingSuperNode { .. } => {
                forwarding_super_node::build_node(self.as_forwarding_super_node().as_ref(), context)
            }
            Node::GlobalVariableAndWriteNode { .. } => {
                global_variable_and_write_node::build_node(self.as_global_variable_and_write_node().as_ref(), context)
            }
            Node::GlobalVariableOperatorWriteNode { .. } => {
                global_variable_operator_write_node::build_node(self.as_global_variable_operator_write_node().as_ref(), context)
            }
            Node::GlobalVariableOrWriteNode { .. } => {
                global_variable_or_write_node::build_node(self.as_global_variable_or_write_node().as_ref(), context)
            }
            Node::GlobalVariableReadNode { .. } => {
                global_variable_read_node::build_node(self.as_global_variable_read_node().as_ref(), context)
            }
            Node::GlobalVariableTargetNode { .. } => {
                global_variable_target_node::build_node(self.as_global_variable_target_node().as_ref(), context)
            }
            Node::GlobalVariableWriteNode { .. } => {
                global_variable_write_node::build_node(self.as_global_variable_write_node().as_ref(), context)
            }
            Node::HashNode { .. } => {
                hash_node::build_node(self.as_hash_node().as_ref(), context)
            }
            Node::HashPatternNode { .. } => {
                hash_pattern_node::build_node(self.as_hash_pattern_node().as_ref(), context)
            }
            Node::IfNode { .. } => {
                if_node::build_node(self.as_if_node().as_ref(), context)
            }
            Node::ImaginaryNode { .. } => {
                imaginary_node::build_node(self.as_imaginary_node().as_ref(), context)
            }
            Node::ImplicitNode { .. } => {
                implicit_node::build_node(self.as_implicit_node().as_ref(), context)
            }
            Node::ImplicitRestNode { .. } => {
                implicit_rest_node::build_node(self.as_implicit_rest_node().as_ref(), context)
            }
            Node::InNode { .. } => {
                in_node::build_node(self.as_in_node().as_ref(), context)
            }
            Node::IndexAndWriteNode { .. } => {
                index_and_write_node::build_node(self.as_index_and_write_node().as_ref(), context)
            }
            Node::IndexOperatorWriteNode { .. } => {
                index_operator_write_node::build_node(self.as_index_operator_write_node().as_ref(), context)
            }
            Node::IndexOrWriteNode { .. } => {
                index_or_write_node::build_node(self.as_index_or_write_node().as_ref(), context)
            }
            Node::IndexTargetNode { .. } => {
                index_target_node::build_node(self.as_index_target_node().as_ref(), context)
            }
            Node::InstanceVariableAndWriteNode { .. } => {
                instance_variable_and_write_node::build_node(self.as_instance_variable_and_write_node().as_ref(), context)
            }
            Node::InstanceVariableOperatorWriteNode { .. } => {
                instance_variable_operator_write_node::build_node(self.as_instance_variable_operator_write_node().as_ref(), context)
            }
            Node::InstanceVariableOrWriteNode { .. } => {
                instance_variable_or_write_node::build_node(self.as_instance_variable_or_write_node().as_ref(), context)
            }
            Node::InstanceVariableReadNode { .. } => {
                instance_variable_read_node::build_node(self.as_instance_variable_read_node().as_ref(), context)
            }
            Node::InstanceVariableTargetNode { .. } => {
                instance_variable_target_node::build_node(self.as_instance_variable_target_node().as_ref(), context)
            }
            Node::InstanceVariableWriteNode { .. } => {
                instance_variable_write_node::build_node(self.as_instance_variable_write_node().as_ref(), context)
            }
            Node::IntegerNode { .. } => {
                integer_node::build_node(self.as_integer_node().as_ref(), context)
            }
            Node::InterpolatedMatchLastLineNode { .. } => {
                interpolated_match_last_line_node::build_node(self.as_interpolated_match_last_line_node().as_ref(), context)
            }
            Node::InterpolatedRegularExpressionNode { .. } => {
                interpolated_regular_expression_node::build_node(self.as_interpolated_regular_expression_node().as_ref(), context)
            }
            Node::InterpolatedStringNode { .. } => {
                interpolated_string_node::build_node(self.as_interpolated_string_node().as_ref(), context)
            }
            Node::InterpolatedSymbolNode { .. } => {
                interpolated_symbol_node::build_node(self.as_interpolated_symbol_node().as_ref(), context)
            }
            Node::InterpolatedXStringNode { .. } => {
                interpolated_x_string_node::build_node(self.as_interpolated_x_string_node().as_ref(), context)
            }
            Node::ItLocalVariableReadNode { .. } => {
                it_local_variable_read_node::build_node(self.as_it_local_variable_read_node().as_ref(), context)
            }
            Node::ItParametersNode { .. } => {
                it_parameters_node::build_node(self.as_it_parameters_node().as_ref(), context)
            }
            Node::KeywordHashNode { .. } => {
                keyword_hash_node::build_node(self.as_keyword_hash_node().as_ref(), context)
            }
            Node::KeywordRestParameterNode { .. } => {
                keyword_rest_parameter_node::build_node(self.as_keyword_rest_parameter_node().as_ref(), context)
            }
            Node::LambdaNode { .. } => {
                lambda_node::build_node(self.as_lambda_node().as_ref(), context)
            }
            Node::LocalVariableAndWriteNode { .. } => {
                local_variable_and_write_node::build_node(self.as_local_variable_and_write_node().as_ref(), context)
            }
            Node::LocalVariableOperatorWriteNode { .. } => {
                local_variable_operator_write_node::build_node(self.as_local_variable_operator_write_node().as_ref(), context)
            }
            Node::LocalVariableOrWriteNode { .. } => {
                local_variable_or_write_node::build_node(self.as_local_variable_or_write_node().as_ref(), context)
            }
            Node::LocalVariableReadNode { .. } => {
                local_variable_read_node::build_node(self.as_local_variable_read_node().as_ref(), context)
            }
            Node::LocalVariableTargetNode { .. } => {
                local_variable_target_node::build_node(self.as_local_variable_target_node().as_ref(), context)
            }
            Node::LocalVariableWriteNode { .. } => {
                local_variable_write_node::build_node(self.as_local_variable_write_node().as_ref(), context)
            }
            Node::MatchLastLineNode { .. } => {
                match_last_line_node::build_node(self.as_match_last_line_node().as_ref(), context)
            }
            Node::MatchPredicateNode { .. } => {
                match_predicate_node::build_node(self.as_match_predicate_node().as_ref(), context)
            }
            Node::MatchRequiredNode { .. } => {
                match_required_node::build_node(self.as_match_required_node().as_ref(), context)
            }
            Node::MatchWriteNode { .. } => {
                match_write_node::build_node(self.as_match_write_node().as_ref(), context)
            }
            Node::MissingNode { .. } => {
                missing_node::build_node(self.as_missing_node().as_ref(), context)
            }
            Node::ModuleNode { .. } => {
                module_node::build_node(self.as_module_node().as_ref(), context)
            }
            Node::MultiTargetNode { .. } => {
                multi_target_node::build_node(self.as_multi_target_node().as_ref(), context)
            }
            Node::MultiWriteNode { .. } => {
                multi_write_node::build_node(self.as_multi_write_node().as_ref(), context)
            }
            Node::NextNode { .. } => {
                next_node::build_node(self.as_next_node().as_ref(), context)
            }
            Node::NilNode { .. } => {
                nil_node::build_node(self.as_nil_node().as_ref(), context)
            }
            Node::NoKeywordsParameterNode { .. } => {
                no_keywords_parameter_node::build_node(self.as_no_keywords_parameter_node().as_ref(), context)
            }
            Node::NumberedParametersNode { .. } => {
                numbered_parameters_node::build_node(self.as_numbered_parameters_node().as_ref(), context)
            }
            Node::NumberedReferenceReadNode { .. } => {
                numbered_reference_read_node::build_node(self.as_numbered_reference_read_node().as_ref(), context)
            }
            Node::OptionalKeywordParameterNode { .. } => {
                optional_keyword_parameter_node::build_node(self.as_optional_keyword_parameter_node().as_ref(), context)
            }
            Node::OptionalParameterNode { .. } => {
                optional_parameter_node::build_node(self.as_optional_parameter_node().as_ref(), context)
            }
            Node::OrNode { .. } => {
                or_node::build_node(self.as_or_node().as_ref(), context)
            }
            Node::ParametersNode { .. } => {
                parameters_node::build_node(self.as_parameters_node().as_ref(), context)
            }
            Node::ParenthesesNode { .. } => {
                parentheses_node::build_node(self.as_parentheses_node().as_ref(), context)
            }
            Node::PinnedExpressionNode { .. } => {
                pinned_expression_node::build_node(self.as_pinned_expression_node().as_ref(), context)
            }
            Node::PinnedVariableNode { .. } => {
                pinned_variable_node::build_node(self.as_pinned_variable_node().as_ref(), context)
            }
            Node::PostExecutionNode { .. } => {
                post_execution_node::build_node(self.as_post_execution_node().as_ref(), context)
            }
            Node::PreExecutionNode { .. } => {
                pre_execution_node::build_node(self.as_pre_execution_node().as_ref(), context)
            }
            Node::ProgramNode { .. } => {
                program_node::build_node(self.as_program_node().as_ref(), context)
            }
            Node::RangeNode { .. } => {
                range_node::build_node(self.as_range_node().as_ref(), context)
            }
            Node::RationalNode { .. } => {
                rational_node::build_node(self.as_rational_node().as_ref(), context)
            }
            Node::RedoNode { .. } => {
                redo_node::build_node(self.as_redo_node().as_ref(), context)
            }
            Node::RegularExpressionNode { .. } => {
                regular_expression_node::build_node(self.as_regular_expression_node().as_ref(), context)
            }
            Node::RequiredKeywordParameterNode { .. } => {
                required_keyword_parameter_node::build_node(self.as_required_keyword_parameter_node().as_ref(), context)
            }
            Node::RequiredParameterNode { .. } => {
                required_parameter_node::build_node(self.as_required_parameter_node().as_ref(), context)
            }
            Node::RescueModifierNode { .. } => {
                rescue_modifier_node::build_node(self.as_rescue_modifier_node().as_ref(), context)
            }
            Node::RescueNode { .. } => {
                rescue_node::build_node(self.as_rescue_node().as_ref(), context)
            }
            Node::RestParameterNode { .. } => {
                rest_parameter_node::build_node(self.as_rest_parameter_node().as_ref(), context)
            }
            Node::RetryNode { .. } => {
                retry_node::build_node(self.as_retry_node().as_ref(), context)
            }
            Node::ReturnNode { .. } => {
                return_node::build_node(self.as_return_node().as_ref(), context)
            }
            Node::SelfNode { .. } => {
                self_node::build_node(self.as_self_node().as_ref(), context)
            }
            Node::ShareableConstantNode { .. } => {
                shareable_constant_node::build_node(self.as_shareable_constant_node().as_ref(), context)
            }
            Node::SingletonClassNode { .. } => {
                singleton_class_node::build_node(self.as_singleton_class_node().as_ref(), context)
            }
            Node::SourceEncodingNode { .. } => {
                source_encoding_node::build_node(self.as_source_encoding_node().as_ref(), context)
            }
            Node::SourceFileNode { .. } => {
                source_file_node::build_node(self.as_source_file_node().as_ref(), context)
            }
            Node::SourceLineNode { .. } => {
                source_line_node::build_node(self.as_source_line_node().as_ref(), context)
            }
            Node::SplatNode { .. } => {
                splat_node::build_node(self.as_splat_node().as_ref(), context)
            }
            Node::StatementsNode { .. } => {
                statements_node::build_node(self.as_statements_node().as_ref(), context)
            }
            Node::StringNode { .. } => {
                string_node::build_node(self.as_string_node().as_ref(), context)
            }
            Node::SuperNode { .. } => {
                super_node::build_node(self.as_super_node().as_ref(), context)
            }
            Node::SymbolNode { .. } => {
                symbol_node::build_node(self.as_symbol_node().as_ref(), context)
            }
            Node::TrueNode { .. } => {
                true_node::build_node(self.as_true_node().as_ref(), context)
            }
            Node::UndefNode { .. } => {
                undef_node::build_node(self.as_undef_node().as_ref(), context)
            }
            Node::UnlessNode { .. } => {
                unless_node::build_node(self.as_unless_node().as_ref(), context)
            }
            Node::UntilNode { .. } => {
                until_node::build_node(self.as_until_node().as_ref(), context)
            }
            Node::WhenNode { .. } => {
                when_node::build_node(self.as_when_node().as_ref(), context)
            }
            Node::WhileNode { .. } => {
                while_node::build_node(self.as_while_node().as_ref(), context)
            }
            Node::XStringNode { .. } => {
                x_string_node::build_node(self.as_x_string_node().as_ref(), context)
            }
            Node::YieldNode { .. } => {
                yield_node::build_node(self.as_yield_node().as_ref(), context)
            }
        };
        vec.push(built_node);


        context.is_statement = prev_is_statement;
        context.built_end = context.built_end.max(self.location().end_offset());

        Document::Array(vec)

    }
}

impl BuildPrismNode for Option<Node<'_>> {
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node._build(context),
            None => Document::None,
        }
    }
    fn build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.build(context),
            None => Document::None,
        }
    }
    fn build_with(
        &self,
        context: &mut BuildContext,
        before: Option<Document>,
        after: Option<Document>,
    ) -> Document {
        match self {
            Some(node) => node.build_with(context, before, after),
            None => Document::None,
        }
    }
}
