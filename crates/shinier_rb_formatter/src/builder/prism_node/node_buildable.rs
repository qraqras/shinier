use crate::builder::prism_node::node::*;
use crate::document::Document;
use crate::{Build, builder::builder::hardline};
use ruby_prism::{Comment, Comments, Node};
use std::{io::Read, iter::Peekable};

pub struct BuildContext<'a> {
    pub source: &'a [u8],
    pub built_end: usize,
    pub comments: &'a mut Peekable<Comments<'a>>,
    pub inner_comment: Vec<Comment<'a>>,
    pub leading_line_breaks: bool,
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
    fn _build(&self, context: &mut BuildContext) -> Document {
        #[rustfmt::skip]
        fn build_subnode(node: &Node, context: &mut BuildContext) -> Document {
            match node {
                Node::AliasGlobalVariableNode { .. } => {
                    node.as_alias_global_variable_node().as_ref().build_aaa(context)
                }
                Node::AliasMethodNode { .. } => {
                    node.as_alias_method_node().as_ref().build_aaa(context)
                }
                Node::AlternationPatternNode { .. } => {
                    node.as_alternation_pattern_node().as_ref().build_aaa(context)
                }
                Node::AndNode { .. } => {
                    node.as_and_node().as_ref().build_aaa(context)
                }
                Node::ArgumentsNode { .. } => {
                    node.as_arguments_node().as_ref().build_aaa(context)
                }
                Node::ArrayNode { .. } => {
                    node.as_array_node().as_ref().build_aaa(context)
                }
                Node::ArrayPatternNode { .. } => {
                    node.as_array_pattern_node().as_ref().build_aaa(context)
                }
                Node::AssocNode { .. } => {
                    node.as_assoc_node().as_ref().build_aaa(context)
                }
                Node::AssocSplatNode { .. } => {
                    node.as_assoc_splat_node().as_ref().build_aaa(context)
                }
                Node::BackReferenceReadNode { .. } => {
                    node.as_back_reference_read_node().as_ref().build_aaa(context)
                }
                Node::BeginNode { .. } => {
                    node.as_begin_node().as_ref().build_aaa(context)
                }
                Node::BlockArgumentNode { .. } => {
                    node.as_block_argument_node().as_ref().build_aaa(context)
                }
                Node::BlockLocalVariableNode { .. } => {
                    node.as_block_local_variable_node().as_ref().build_aaa(context)
                }
                Node::BlockNode { .. } => {
                    node.as_block_node().as_ref().build_aaa(context)
                }
                Node::BlockParameterNode { .. } => {
                    node.as_block_parameter_node().as_ref().build_aaa(context)
                }
                Node::BlockParametersNode { .. } => {
                    node.as_block_parameters_node().as_ref().build_aaa(context)
                }
                Node::BreakNode { .. } => {
                    node.as_break_node().as_ref().build_aaa(context)
                }
                Node::CallAndWriteNode { .. } => {
                    node.as_call_and_write_node().as_ref().build_aaa(context)
                }
                Node::CallNode { .. } => {
                    node.as_call_node().as_ref().build_aaa(context)
                }
                Node::CallOperatorWriteNode { .. } => {
                    node.as_call_operator_write_node().as_ref().build_aaa(context)
                }
                Node::CallOrWriteNode { .. } => {
                    node.as_call_or_write_node().as_ref().build_aaa(context)
                }
                Node::CallTargetNode { .. } => {
                    node.as_call_target_node().as_ref().build_aaa(context)
                }
                Node::CapturePatternNode { .. } => {
                    node.as_capture_pattern_node().as_ref().build_aaa(context)
                }
                Node::CaseMatchNode { .. } => {
                    node.as_case_match_node().as_ref().build_aaa(context)
                }
                Node::CaseNode { .. } => {
                    node.as_case_node().as_ref().build_aaa(context)
                }
                Node::ClassNode { .. } => {
                    node.as_class_node().as_ref().build_aaa(context)
                }
                Node::ClassVariableAndWriteNode { .. } => {
                    node.as_class_variable_and_write_node().as_ref().build_aaa(context)
                }
                Node::ClassVariableOperatorWriteNode { .. } => {
                    node.as_class_variable_operator_write_node().as_ref().build_aaa(context)
                }
                Node::ClassVariableOrWriteNode { .. } => {
                    node.as_class_variable_or_write_node().as_ref().build_aaa(context)
                }
                Node::ClassVariableReadNode { .. } => {
                    node.as_class_variable_read_node().as_ref().build_aaa(context)
                }
                Node::ClassVariableTargetNode { .. } => {
                    node.as_class_variable_target_node().as_ref().build_aaa(context)
                }
                Node::ClassVariableWriteNode { .. } => {
                    node.as_class_variable_write_node().as_ref().build_aaa(context)
                }
                Node::ConstantAndWriteNode { .. } => {
                    node.as_constant_and_write_node().as_ref().build_aaa(context)
                }
                Node::ConstantOperatorWriteNode { .. } => {
                    node.as_constant_operator_write_node().as_ref().build_aaa(context)
                }
                Node::ConstantOrWriteNode { .. } => {
                    node.as_constant_or_write_node().as_ref().build_aaa(context)
                }
                Node::ConstantPathAndWriteNode { .. } => {
                    node.as_constant_path_and_write_node().as_ref().build_aaa(context)
                }
                Node::ConstantPathNode { .. } => {
                    node.as_constant_path_node().as_ref().build_aaa(context)
                }
                Node::ConstantPathOperatorWriteNode { .. } => {
                    node.as_constant_path_operator_write_node().as_ref().build_aaa(context)
                }
                Node::ConstantPathOrWriteNode { .. } => {
                    node.as_constant_path_or_write_node().as_ref().build_aaa(context)
                }
                Node::ConstantPathTargetNode { .. } => {
                    node.as_constant_path_target_node().as_ref().build_aaa(context)
                }
                Node::ConstantPathWriteNode { .. } => {
                    node.as_constant_path_write_node().as_ref().build_aaa(context)
                }
                Node::ConstantReadNode { .. } => {
                    node.as_constant_read_node().as_ref().build_aaa(context)
                }
                Node::ConstantTargetNode { .. } => {
                    node.as_constant_target_node().as_ref().build_aaa(context)
                }
                Node::ConstantWriteNode { .. } => {
                    node.as_constant_write_node().as_ref().build_aaa(context)
                }
                Node::DefNode { .. } => {
                    node.as_def_node().as_ref().build_aaa(context)
                }
                Node::DefinedNode { .. } => {
                    node.as_defined_node().as_ref().build_aaa(context)
                }
                Node::ElseNode { .. } => {
                    node.as_else_node().as_ref().build_aaa(context)
                }
                Node::EmbeddedStatementsNode { .. } => {
                    node.as_embedded_statements_node().as_ref().build_aaa(context)
                }
                Node::EmbeddedVariableNode { .. } => {
                    node.as_embedded_variable_node().as_ref().build_aaa(context)
                }
                Node::EnsureNode { .. } => {
                    node.as_ensure_node().as_ref().build_aaa(context)
                }
                Node::FalseNode { .. } => {
                    node.as_false_node().as_ref().build_aaa(context)
                }
                Node::FindPatternNode { .. } => {
                    node.as_find_pattern_node().as_ref().build_aaa(context)
                }
                Node::FlipFlopNode { .. } => {
                    node.as_flip_flop_node().as_ref().build_aaa(context)
                }
                Node::FloatNode { .. } => {
                    node.as_float_node().as_ref().build_aaa(context)
                }
                Node::ForNode { .. } => {
                    node.as_for_node().as_ref().build_aaa(context)
                }
                Node::ForwardingArgumentsNode { .. } => {
                    node.as_forwarding_arguments_node().as_ref().build_aaa(context)
                }
                Node::ForwardingParameterNode { .. } => {
                    node.as_forwarding_parameter_node().as_ref().build_aaa(context)
                }
                Node::ForwardingSuperNode { .. } => {
                    node.as_forwarding_super_node().as_ref().build_aaa(context)
                }
                Node::GlobalVariableAndWriteNode { .. } => {
                    node.as_global_variable_and_write_node().as_ref().build_aaa(context)
                }
                Node::GlobalVariableOperatorWriteNode { .. } => {
                    node.as_global_variable_operator_write_node().as_ref().build_aaa(context)
                }
                Node::GlobalVariableOrWriteNode { .. } => {
                    node.as_global_variable_or_write_node().as_ref().build_aaa(context)
                }
                Node::GlobalVariableReadNode { .. } => {
                    node.as_global_variable_read_node().as_ref().build_aaa(context)
                }
                Node::GlobalVariableTargetNode { .. } => {
                    node.as_global_variable_target_node().as_ref().build_aaa(context)
                }
                Node::GlobalVariableWriteNode { .. } => {
                    node.as_global_variable_write_node().as_ref().build_aaa(context)
                }
                Node::HashNode { .. } => {
                    node.as_hash_node().as_ref().build_aaa(context)
                }
                Node::HashPatternNode { .. } => {
                    node.as_hash_pattern_node().as_ref().build_aaa(context)
                }
                Node::IfNode { .. } => {
                    node.as_if_node().as_ref().build_aaa(context)
                }
                Node::ImaginaryNode { .. } => {
                    node.as_imaginary_node().as_ref().build_aaa(context)
                }
                Node::ImplicitNode { .. } => {
                    node.as_implicit_node().as_ref().build_aaa(context)
                }
                Node::ImplicitRestNode { .. } => {
                    node.as_implicit_rest_node().as_ref().build_aaa(context)
                }
                Node::InNode { .. } => {
                    node.as_in_node().as_ref().build_aaa(context)
                }
                Node::IndexAndWriteNode { .. } => {
                    node.as_index_and_write_node().as_ref().build_aaa(context)
                }
                Node::IndexOperatorWriteNode { .. } => {
                    node.as_index_operator_write_node().as_ref().build_aaa(context)
                }
                Node::IndexOrWriteNode { .. } => {
                    node.as_index_or_write_node().as_ref().build_aaa(context)
                }
                Node::IndexTargetNode { .. } => {
                    node.as_index_target_node().as_ref().build_aaa(context)
                }
                Node::InstanceVariableAndWriteNode { .. } => {
                    node.as_instance_variable_and_write_node().as_ref().build_aaa(context)
                }
                Node::InstanceVariableOperatorWriteNode { .. } => {
                    node.as_instance_variable_operator_write_node().as_ref().build_aaa(context)
                }
                Node::InstanceVariableOrWriteNode { .. } => {
                    node.as_instance_variable_or_write_node().as_ref().build_aaa(context)
                }
                Node::InstanceVariableReadNode { .. } => {
                    node.as_instance_variable_read_node().as_ref().build_aaa(context)
                }
                Node::InstanceVariableTargetNode { .. } => {
                    node.as_instance_variable_target_node().as_ref().build_aaa(context)
                }
                Node::InstanceVariableWriteNode { .. } => {
                    node.as_instance_variable_write_node().as_ref().build_aaa(context)
                }
                Node::IntegerNode { .. } => {
                    node.as_integer_node().as_ref().build_aaa(context)
                }
                Node::InterpolatedMatchLastLineNode { .. } => {
                    node.as_interpolated_match_last_line_node().as_ref().build_aaa(context)
                }
                Node::InterpolatedRegularExpressionNode { .. } => {
                    node.as_interpolated_regular_expression_node().as_ref().build_aaa(context)
                }
                Node::InterpolatedStringNode { .. } => {
                    node.as_interpolated_string_node().as_ref().build_aaa(context)
                }
                Node::InterpolatedSymbolNode { .. } => {
                    node.as_interpolated_symbol_node().as_ref().build_aaa(context)
                }
                Node::InterpolatedXStringNode { .. } => {
                    node.as_interpolated_x_string_node().as_ref().build_aaa(context)
                }
                Node::ItLocalVariableReadNode { .. } => {
                    node.as_it_local_variable_read_node().as_ref().build_aaa(context)
                }
                Node::ItParametersNode { .. } => {
                    node.as_it_parameters_node().as_ref().build_aaa(context)
                }
                Node::KeywordHashNode { .. } => {
                    node.as_keyword_hash_node().as_ref().build_aaa(context)
                }
                Node::KeywordRestParameterNode { .. } => {
                    node.as_keyword_rest_parameter_node().as_ref().build_aaa(context)
                }
                Node::LambdaNode { .. } => {
                    node.as_lambda_node().as_ref().build_aaa(context)
                }
                Node::LocalVariableAndWriteNode { .. } => {
                    node.as_local_variable_and_write_node().as_ref().build_aaa(context)
                }
                Node::LocalVariableOperatorWriteNode { .. } => {
                    node.as_local_variable_operator_write_node().as_ref().build_aaa(context)
                }
                Node::LocalVariableOrWriteNode { .. } => {
                    node.as_local_variable_or_write_node().as_ref().build_aaa(context)
                }
                Node::LocalVariableReadNode { .. } => {
                    node.as_local_variable_read_node().as_ref().build_aaa(context)
                }
                Node::LocalVariableTargetNode { .. } => {
                    node.as_local_variable_target_node().as_ref().build_aaa(context)
                }
                Node::LocalVariableWriteNode { .. } => {
                    node.as_local_variable_write_node().as_ref().build_aaa(context)
                }
                Node::MatchLastLineNode { .. } => {
                    node.as_match_last_line_node().as_ref().build_aaa(context)
                }
                Node::MatchPredicateNode { .. } => {
                    node.as_match_predicate_node().as_ref().build_aaa(context)
                }
                Node::MatchRequiredNode { .. } => {
                    node.as_match_required_node().as_ref().build_aaa(context)
                }
                Node::MatchWriteNode { .. } => {
                    node.as_match_write_node().as_ref().build_aaa(context)
                }
                Node::MissingNode { .. } => {
                    node.as_missing_node().as_ref().build_aaa(context)
                }
                Node::ModuleNode { .. } => {
                    node.as_module_node().as_ref().build_aaa(context)
                }
                Node::MultiTargetNode { .. } => {
                    node.as_multi_target_node().as_ref().build_aaa(context)
                }
                Node::MultiWriteNode { .. } => {
                    node.as_multi_write_node().as_ref().build_aaa(context)
                }
                Node::NextNode { .. } => {
                    node.as_next_node().as_ref().build_aaa(context)
                }
                Node::NilNode { .. } => {
                    node.as_nil_node().as_ref().build_aaa(context)
                }
                Node::NoKeywordsParameterNode { .. } => {
                    node.as_no_keywords_parameter_node().as_ref().build_aaa(context)
                }
                Node::NumberedParametersNode { .. } => {
                    node.as_numbered_parameters_node().as_ref().build_aaa(context)
                }
                Node::NumberedReferenceReadNode { .. } => {
                    node.as_numbered_reference_read_node().as_ref().build_aaa(context)
                }
                Node::OptionalKeywordParameterNode { .. } => {
                    node.as_optional_keyword_parameter_node().as_ref().build_aaa(context)
                }
                Node::OptionalParameterNode { .. } => {
                    node.as_optional_parameter_node().as_ref().build_aaa(context)
                }
                Node::OrNode { .. } => {
                    node.as_or_node().as_ref().build_aaa(context)
                }
                Node::ParametersNode { .. } => {
                    node.as_parameters_node().as_ref().build_aaa(context)
                }
                Node::ParenthesesNode { .. } => {
                    node.as_parentheses_node().as_ref().build_aaa(context)
                }
                Node::PinnedExpressionNode { .. } => {
                    node.as_pinned_expression_node().as_ref().build_aaa(context)
                }
                Node::PinnedVariableNode { .. } => {
                    node.as_pinned_variable_node().as_ref().build_aaa(context)
                }
                Node::PostExecutionNode { .. } => {
                    node.as_post_execution_node().as_ref().build_aaa(context)
                }
                Node::PreExecutionNode { .. } => {
                    node.as_pre_execution_node().as_ref().build_aaa(context)
                }
                Node::ProgramNode { .. } => {
                    node.as_program_node().as_ref().build_aaa(context)
                }
                Node::RangeNode { .. } => {
                    node.as_range_node().as_ref().build_aaa(context)
                }
                Node::RationalNode { .. } => {
                    node.as_rational_node().as_ref().build_aaa(context)
                }
                Node::RedoNode { .. } => {
                    node.as_redo_node().as_ref().build_aaa(context)
                }
                Node::RegularExpressionNode { .. } => {
                    node.as_regular_expression_node().as_ref().build_aaa(context)
                }
                Node::RequiredKeywordParameterNode { .. } => {
                    node.as_required_keyword_parameter_node().as_ref().build_aaa(context)
                }
                Node::RequiredParameterNode { .. } => {
                    node.as_required_parameter_node().as_ref().build_aaa(context)
                }
                Node::RescueModifierNode { .. } => {
                    node.as_rescue_modifier_node().as_ref().build_aaa(context)
                }
                Node::RescueNode { .. } => {
                    node.as_rescue_node().as_ref().build_aaa(context)
                }
                Node::RestParameterNode { .. } => {
                    node.as_rest_parameter_node().as_ref().build_aaa(context)
                }
                Node::RetryNode { .. } => {
                    node.as_retry_node().as_ref().build_aaa(context)
                }
                Node::ReturnNode { .. } => {
                    node.as_return_node().as_ref().build_aaa(context)
                }
                Node::SelfNode { .. } => {
                    node.as_self_node().as_ref().build_aaa(context)
                }
                Node::ShareableConstantNode { .. } => {
                    node.as_shareable_constant_node().as_ref().build_aaa(context)
                }
                Node::SingletonClassNode { .. } => {
                    node.as_singleton_class_node().as_ref().build_aaa(context)
                }
                Node::SourceEncodingNode { .. } => {
                    node.as_source_encoding_node().as_ref().build_aaa(context)
                }
                Node::SourceFileNode { .. } => {
                    node.as_source_file_node().as_ref().build_aaa(context)
                }
                Node::SourceLineNode { .. } => {
                    node.as_source_line_node().as_ref().build_aaa(context)
                }
                Node::SplatNode { .. } => {
                    node.as_splat_node().as_ref().build_aaa(context)
                }
                Node::StatementsNode { .. } => {
                    node.as_statements_node().as_ref().build_aaa(context)
                }
                Node::StringNode { .. } => {
                    node.as_string_node().as_ref().build_aaa(context)
                }
                Node::SuperNode { .. } => {
                    node.as_super_node().as_ref().build_aaa(context)
                }
                Node::SymbolNode { .. } => {
                    node.as_symbol_node().as_ref().build_aaa(context)
                }
                Node::TrueNode { .. } => {
                    node.as_true_node().as_ref().build_aaa(context)
                }
                Node::UndefNode { .. } => {
                    node.as_undef_node().as_ref().build_aaa(context)
                }
                Node::UnlessNode { .. } => {
                    node.as_unless_node().as_ref().build_aaa(context)
                }
                Node::UntilNode { .. } => {
                    node.as_until_node().as_ref().build_aaa(context)
                }
                Node::WhenNode { .. } => {
                    node.as_when_node().as_ref().build_aaa(context)
                }
                Node::WhileNode { .. } => {
                    node.as_while_node().as_ref().build_aaa(context)
                }
                Node::XStringNode { .. } => {
                    node.as_x_string_node().as_ref().build_aaa(context)
                }
                Node::YieldNode { .. } => {
                    node.as_yield_node().as_ref().build_aaa(context)
                }
            }
        }

        let mut vec = Vec::new();

        // Leading Line Breaks
        if context.leading_line_breaks {
            if let Some(breaks) =
                build_leading_line_breaks(context, self.location().start_offset(), 1usize)
            {
                vec.push(breaks);
            }
        }
        let prev_is_statement = context.leading_line_breaks;
        context.leading_line_breaks = match self {
            Node::StatementsNode { .. } => true,
            Node::ProgramNode { .. } => true,
            _ => false,
        };

        // Leading Comments
        loop {
            let next_comment_start = context.comments.peek().map(|c| c.location().start_offset());
            match next_comment_start {
                Some(start) => {
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

        // // Inner Comments
        // let mut inner_comment = Vec::new();
        // loop {
        //     let next_comment_start = context.comments.peek().map(|c| c.location().start_offset());
        //     match next_comment_start {
        //         Some(start) => {
        //             if start < self.location().end_offset() {
        //                 inner_comment.push(context.comments.next().unwrap());
        //             }
        //         }
        //         None => break,
        //     }
        // }
        // context.inner_comment = inner_comment;

        // Subnode
        vec.push(build_subnode(self, context));

        // Trailing Comments
        loop {
            // peek で必要なオフセットだけ取り出して借用を解放
            let next_comment_start = context.comments.peek().map(|c| c.location().start_offset());
            match next_comment_start {
                Some(start) => {
                    let node_end = self.location().end_offset();
                    // node_end から行末（次の '\n' またはソース終端）を探す
                    let mut line_end = node_end;
                    while line_end < context.source.len() && context.source[line_end] != b'\n' {
                        line_end += 1;
                    }

                    // コメントが node_end 以降で行末より前なら trailing comment と判断
                    if start >= node_end && start < line_end {
                        // ここで peek の借用は終了しているので comments.next() して可変操作できる
                        let comment = context.comments.next().unwrap();
                        let mut buf = String::new();
                        comment.text().read_to_string(&mut buf).unwrap();
                        // ノードの後の同一行なので空白を挟んでコメントを出す
                        vec.push(Document::String(" ".to_string()));
                        vec.push(Document::String(buf));
                        // trailing comment は同一行のため hardline を追加しない（必要なら追加）
                        // built_end をコメント末尾まで進める
                        context.built_end =
                            comment.location().end_offset().min(context.source.len());
                        // 続けて同じ行に他のコメントがあるか確認（通常 1 個だがループして安全に）
                        continue;
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        context.leading_line_breaks = prev_is_statement;
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
