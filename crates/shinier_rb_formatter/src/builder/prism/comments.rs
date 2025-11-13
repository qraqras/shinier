// https://github.com/ruby/prism/blob/main/lib/prism/parse_result/comments.rb
use ruby_prism::*;

/// trait for attaching comments to targets
trait Attach<'sh> {
    fn new(target: TargetType<'sh>) -> Self;
    fn start_offset(&self) -> usize;
    fn end_offset(&self) -> usize;
    fn encloses(&self, comment: &Comment<'sh>) -> bool;
    fn leading_comment(&self, comment: &Comment<'sh>);
    fn trailing_comment(&self, comment: &Comment<'sh>);
}

/// target type for attaching comments
#[derive(Clone, Copy)]
enum TargetType<'sh> {
    Node(&'sh Node<'sh>),
    Location(&'sh Location<'sh>),
}

/// target node for attaching comments
#[derive(Clone, Copy)]
pub struct Target<'sh> {
    target: TargetType<'sh>,
}
impl<'sh> Attach<'sh> for Target<'sh> {
    fn new(target: TargetType<'sh>) -> Self {
        Target { target }
    }
    fn start_offset(&self) -> usize {
        match self.target {
            TargetType::Node(n) => n.location().start_offset(),
            TargetType::Location(l) => l.start_offset(),
        }
    }
    fn end_offset(&self) -> usize {
        match self.target {
            TargetType::Node(n) => n.location().end_offset(),
            TargetType::Location(l) => l.end_offset(),
        }
    }
    fn encloses(&self, comment: &Comment<'sh>) -> bool {
        match self.target {
            TargetType::Node(n) => {
                self.start_offset() <= comment.location().start_offset()
                    && comment.location().end_offset() <= self.end_offset()
            }
            TargetType::Location(l) => false,
        }
    }
    fn leading_comment(&self, _comment: &Comment<'sh>) {
        // TODO: implement
    }
    fn trailing_comment(&self, _comment: &Comment<'sh>) {
        // TODO: implement
    }
}

pub fn attach(parse_result: &ParseResult<'_>) {
    let node = &parse_result.node();
    let source = parse_result.source();
    for comment in parse_result.comments() {
        let (preceding, enclosing, following) = nearest_targets(node, &comment);
        if is_trailing(&comment, source) {
            if let Some(preceding) = preceding {
                preceding.trailing_comment(&comment);
            } else {
                if let Some(following) = following {
                    following.leading_comment(&comment);
                } else if let Some(enclosing) = enclosing {
                    enclosing.leading_comment(&comment);
                } else {
                    Target::new(TargetType::Node(node)).leading_comment(&comment);
                }
            }
        } else {
            if let Some(following) = following {
                following.leading_comment(&comment);
            } else if let Some(preceding) = preceding {
                preceding.trailing_comment(&comment);
            } else {
                if let Some(enclosing) = enclosing {
                    enclosing.leading_comment(&comment);
                } else {
                    Target::new(TargetType::Node(node)).leading_comment(&comment);
                }
            }
        }
    }
}

fn nearest_targets<'sh>(
    node: &'sh Node<'sh>,
    comment: &'sh Comment<'sh>,
) -> (
    Option<Target<'sh>>,
    Option<Target<'sh>>,
    Option<Target<'sh>>,
) {
    let comment_start = comment.location().start_offset();
    let comment_end = comment.location().end_offset();

    let mut targets = comment_targets(node);

    targets.sort_by_key(|t| t.start_offset());
    let mut preceding = None;
    let mut following = None;

    let mut left = 0;
    let mut right = targets.len();

    while left < right {
        let middle = (left + right) / 2;
        let target = &targets[middle];

        let target_start = target.start_offset();
        let target_end = target.end_offset();

        if target.encloses(comment) {
            match target.target {
                TargetType::Node(n) => {
                    return nearest_targets(n, comment);
                }
                TargetType::Location(_l) => {
                    unreachable!("location target should not enclose comments")
                }
            }
        }

        if target_end <= comment_start {
            preceding = Some(*target);
            left = middle + 1;
            continue;
        }

        if comment_end <= target_start {
            following = Some(*target);
            right = middle;
            continue;
        }
        unreachable!("comment location overlaps with a target location");
    }

    (
        preceding,
        Some(Target::new(TargetType::Node(node))),
        following,
    )
}

fn is_trailing(comment: &Comment, source: &[u8]) -> bool {
    let mut idx = comment.location().start_offset();
    while idx > 0 {
        if source[idx - 1] == b' ' || source[idx - 1] == b'\t' {
            idx -= 1;
            continue;
        }
        if source[idx - 1] == b'\n' {
            return false;
        }
        return true;
    }
    false
}

#[rustfmt::skip]
fn comment_targets<'sh>(node: &'sh Node<'sh>) -> Vec<Target<'sh>> {
    match node {
        Node::AliasGlobalVariableNode           { .. } => comment_targets_of_alias_global_variable_node           (node.as_alias_global_variable_node().unwrap()           ),
        Node::AliasMethodNode                   { .. } => comment_targets_of_alias_method_node                    (node.as_alias_method_node().unwrap()                    ),
        Node::AlternationPatternNode            { .. } => comment_targets_of_alternation_pattern_node             (node.as_alternation_pattern_node().unwrap()             ),
        Node::AndNode                           { .. } => comment_targets_of_and_node                             (node.as_and_node().unwrap()                             ),
        Node::ArgumentsNode                     { .. } => comment_targets_of_arguments_node                       (node.as_arguments_node().unwrap()                       ),
        Node::ArrayNode                         { .. } => comment_targets_of_array_node                           (node.as_array_node().unwrap()                           ),
        Node::ArrayPatternNode                  { .. } => comment_targets_of_array_pattern_node                   (node.as_array_pattern_node().unwrap()                   ),
        Node::AssocNode                         { .. } => comment_targets_of_assoc_node                           (node.as_assoc_node().unwrap()                           ),
        Node::AssocSplatNode                    { .. } => comment_targets_of_assoc_splat_node                     (node.as_assoc_splat_node().unwrap()                     ),
        Node::BackReferenceReadNode             { .. } => comment_targets_of_back_reference_read_node             (node.as_back_reference_read_node().unwrap()             ),
        Node::BeginNode                         { .. } => comment_targets_of_begin_node                           (node.as_begin_node().unwrap()                           ),
        Node::BlockArgumentNode                 { .. } => comment_targets_of_block_argument_node                  (node.as_block_argument_node().unwrap()                  ),
        Node::BlockLocalVariableNode            { .. } => comment_targets_of_block_local_variable_node            (node.as_block_local_variable_node().unwrap()            ),
        Node::BlockNode                         { .. } => comment_targets_of_block_node                           (node.as_block_node().unwrap()                           ),
        Node::BlockParameterNode                { .. } => comment_targets_of_block_parameter_node                 (node.as_block_parameter_node().unwrap()                 ),
        Node::BlockParametersNode               { .. } => comment_targets_of_block_parameters_node                (node.as_block_parameters_node().unwrap()                ),
        Node::BreakNode                         { .. } => comment_targets_of_break_node                           (node.as_break_node().unwrap()                           ),
        Node::CallAndWriteNode                  { .. } => comment_targets_of_call_and_write_node                  (node.as_call_and_write_node().unwrap()                  ),
        Node::CallNode                          { .. } => comment_targets_of_call_node                            (node.as_call_node().unwrap()                            ),
        Node::CallOperatorWriteNode             { .. } => comment_targets_of_call_operator_write_node             (node.as_call_operator_write_node().unwrap()             ),
        Node::CallOrWriteNode                   { .. } => comment_targets_of_call_or_write_node                   (node.as_call_or_write_node().unwrap()                   ),
        Node::CallTargetNode                    { .. } => comment_targets_of_call_target_node                     (node.as_call_target_node().unwrap()                     ),
        Node::CapturePatternNode                { .. } => comment_targets_of_capture_pattern_node                 (node.as_capture_pattern_node().unwrap()                 ),
        Node::CaseMatchNode                     { .. } => comment_targets_of_case_match_node                      (node.as_case_match_node().unwrap()                      ),
        Node::CaseNode                          { .. } => comment_targets_of_case_node                            (node.as_case_node().unwrap()                            ),
        Node::ClassNode                         { .. } => comment_targets_of_class_node                           (node.as_class_node().unwrap()                           ),
        Node::ClassVariableAndWriteNode         { .. } => comment_targets_of_class_variable_and_write_node        (node.as_class_variable_and_write_node().unwrap()        ),
        Node::ClassVariableOperatorWriteNode    { .. } => comment_targets_of_class_variable_operator_write_node   (node.as_class_variable_operator_write_node().unwrap()   ),
        Node::ClassVariableOrWriteNode          { .. } => comment_targets_of_class_variable_or_write_node         (node.as_class_variable_or_write_node().unwrap()         ),
        Node::ClassVariableReadNode             { .. } => comment_targets_of_class_variable_read_node             (node.as_class_variable_read_node().unwrap()             ),
        Node::ClassVariableTargetNode           { .. } => comment_targets_of_class_variable_target_node           (node.as_class_variable_target_node().unwrap()           ),
        Node::ClassVariableWriteNode            { .. } => comment_targets_of_class_variable_write_node            (node.as_class_variable_write_node().unwrap()            ),
        Node::ConstantAndWriteNode              { .. } => comment_targets_of_constant_and_write_node              (node.as_constant_and_write_node().unwrap()              ),
        Node::ConstantOperatorWriteNode         { .. } => comment_targets_of_constant_operator_write_node         (node.as_constant_operator_write_node().unwrap()         ),
        Node::ConstantOrWriteNode               { .. } => comment_targets_of_constant_or_write_node               (node.as_constant_or_write_node().unwrap()               ),
        Node::ConstantPathAndWriteNode          { .. } => comment_targets_of_constant_path_and_write_node         (node.as_constant_path_and_write_node().unwrap()         ),
        Node::ConstantPathNode                  { .. } => comment_targets_of_constant_path_node                   (node.as_constant_path_node().unwrap()                   ),
        Node::ConstantPathOperatorWriteNode     { .. } => comment_targets_of_constant_path_operator_write_node    (node.as_constant_path_operator_write_node().unwrap()    ),
        Node::ConstantPathOrWriteNode           { .. } => comment_targets_of_constant_path_or_write_node          (node.as_constant_path_or_write_node().unwrap()          ),
        Node::ConstantPathTargetNode            { .. } => comment_targets_of_constant_path_target_node            (node.as_constant_path_target_node().unwrap()            ),
        Node::ConstantPathWriteNode             { .. } => comment_targets_of_constant_path_write_node             (node.as_constant_path_write_node().unwrap()             ),
        Node::ConstantReadNode                  { .. } => comment_targets_of_constant_read_node                   (node.as_constant_read_node().unwrap()                   ),
        Node::ConstantTargetNode                { .. } => comment_targets_of_constant_target_node                 (node.as_constant_target_node().unwrap()                 ),
        Node::ConstantWriteNode                 { .. } => comment_targets_of_constant_write_node                  (node.as_constant_write_node().unwrap()                  ),
        Node::DefNode                           { .. } => comment_targets_of_def_node                             (node.as_def_node().unwrap()                             ),
        Node::DefinedNode                       { .. } => comment_targets_of_defined_node                         (node.as_defined_node().unwrap()                         ),
        Node::ElseNode                          { .. } => comment_targets_of_else_node                            (node.as_else_node().unwrap()                            ),
        Node::EmbeddedStatementsNode            { .. } => comment_targets_of_embedded_statements_node             (node.as_embedded_statements_node().unwrap()             ),
        Node::EmbeddedVariableNode              { .. } => comment_targets_of_embedded_variable_node               (node.as_embedded_variable_node().unwrap()               ),
        Node::EnsureNode                        { .. } => comment_targets_of_ensure_node                          (node.as_ensure_node().unwrap()                          ),
        Node::FalseNode                         { .. } => comment_targets_of_false_node                           (node.as_false_node().unwrap()                           ),
        Node::FindPatternNode                   { .. } => comment_targets_of_find_pattern_node                    (node.as_find_pattern_node().unwrap()                    ),
        Node::FlipFlopNode                      { .. } => comment_targets_of_flip_flop_node                       (node.as_flip_flop_node().unwrap()                       ),
        Node::FloatNode                         { .. } => comment_targets_of_float_node                           (node.as_float_node().unwrap()                           ),
        Node::ForNode                           { .. } => comment_targets_of_for_node                             (node.as_for_node().unwrap()                             ),
        Node::ForwardingArgumentsNode           { .. } => comment_targets_of_forwarding_arguments_node            (node.as_forwarding_arguments_node().unwrap()            ),
        Node::ForwardingParameterNode           { .. } => comment_targets_of_forwarding_parameter_node            (node.as_forwarding_parameter_node().unwrap()            ),
        Node::ForwardingSuperNode               { .. } => comment_targets_of_forwarding_super_node                (node.as_forwarding_super_node().unwrap()                ),
        Node::GlobalVariableAndWriteNode        { .. } => comment_targets_of_global_variable_and_write_node       (node.as_global_variable_and_write_node().unwrap()       ),
        Node::GlobalVariableOperatorWriteNode   { .. } => comment_targets_of_global_variable_operator_write_node  (node.as_global_variable_operator_write_node().unwrap()  ),
        Node::GlobalVariableOrWriteNode         { .. } => comment_targets_of_global_variable_or_write_node        (node.as_global_variable_or_write_node().unwrap()        ),
        Node::GlobalVariableReadNode            { .. } => comment_targets_of_global_variable_read_node            (node.as_global_variable_read_node().unwrap()            ),
        Node::GlobalVariableTargetNode          { .. } => comment_targets_of_global_variable_target_node          (node.as_global_variable_target_node().unwrap()          ),
        Node::GlobalVariableWriteNode           { .. } => comment_targets_of_global_variable_write_node           (node.as_global_variable_write_node().unwrap()           ),
        Node::HashNode                          { .. } => comment_targets_of_hash_node                            (node.as_hash_node().unwrap()                            ),
        Node::HashPatternNode                   { .. } => comment_targets_of_hash_pattern_node                    (node.as_hash_pattern_node().unwrap()                    ),
        Node::IfNode                            { .. } => comment_targets_of_if_node                              (node.as_if_node().unwrap()                              ),
        Node::ImaginaryNode                     { .. } => comment_targets_of_imaginary_node                       (node.as_imaginary_node().unwrap()                       ),
        Node::ImplicitNode                      { .. } => comment_targets_of_implicit_node                        (node.as_implicit_node().unwrap()                        ),
        Node::ImplicitRestNode                  { .. } => comment_targets_of_implicit_rest_node                   (node.as_implicit_rest_node().unwrap()                   ),
        Node::InNode                            { .. } => comment_targets_of_in_node                              (node.as_in_node().unwrap()                              ),
        Node::IndexAndWriteNode                 { .. } => comment_targets_of_index_and_write_node                 (node.as_index_and_write_node().unwrap()                 ),
        Node::IndexOperatorWriteNode            { .. } => comment_targets_of_index_operator_write_node            (node.as_index_operator_write_node().unwrap()            ),
        Node::IndexOrWriteNode                  { .. } => comment_targets_of_index_or_write_node                  (node.as_index_or_write_node().unwrap()                  ),
        Node::IndexTargetNode                   { .. } => comment_targets_of_index_target_node                    (node.as_index_target_node().unwrap()                    ),
        Node::InstanceVariableAndWriteNode      { .. } => comment_targets_of_instance_variable_and_write_node     (node.as_instance_variable_and_write_node().unwrap()     ),
        Node::InstanceVariableOperatorWriteNode { .. } => comment_targets_of_instance_variable_operator_write_node(node.as_instance_variable_operator_write_node().unwrap()),
        Node::InstanceVariableOrWriteNode       { .. } => comment_targets_of_instance_variable_or_write_node      (node.as_instance_variable_or_write_node().unwrap()      ),
        Node::InstanceVariableReadNode          { .. } => comment_targets_of_instance_variable_read_node          (node.as_instance_variable_read_node().unwrap()          ),
        Node::InstanceVariableTargetNode        { .. } => comment_targets_of_instance_variable_target_node        (node.as_instance_variable_target_node().unwrap()        ),
        Node::InstanceVariableWriteNode         { .. } => comment_targets_of_instance_variable_write_node         (node.as_instance_variable_write_node().unwrap()         ),
        Node::IntegerNode                       { .. } => comment_targets_of_integer_node                         (node.as_integer_node().unwrap()                         ),
        Node::InterpolatedMatchLastLineNode     { .. } => comment_targets_of_interpolated_match_last_line_node    (node.as_interpolated_match_last_line_node().unwrap()    ),
        Node::InterpolatedRegularExpressionNode { .. } => comment_targets_of_interpolated_regular_expression_node (node.as_interpolated_regular_expression_node().unwrap() ),
        Node::InterpolatedStringNode            { .. } => comment_targets_of_interpolated_string_node             (node.as_interpolated_string_node().unwrap()             ),
        Node::InterpolatedSymbolNode            { .. } => comment_targets_of_interpolated_symbol_node             (node.as_interpolated_symbol_node().unwrap()             ),
        Node::InterpolatedXStringNode           { .. } => comment_targets_of_interpolated_x_string_node           (node.as_interpolated_x_string_node().unwrap()           ),
        Node::ItLocalVariableReadNode           { .. } => comment_targets_of_it_local_variable_read_node          (node.as_it_local_variable_read_node().unwrap()          ),
        Node::ItParametersNode                  { .. } => comment_targets_of_it_parameters_node                   (node.as_it_parameters_node().unwrap()                   ),
        Node::KeywordHashNode                   { .. } => comment_targets_of_keyword_hash_node                    (node.as_keyword_hash_node().unwrap()                    ),
        Node::KeywordRestParameterNode          { .. } => comment_targets_of_keyword_rest_parameter_node          (node.as_keyword_rest_parameter_node().unwrap()          ),
        Node::LambdaNode                        { .. } => comment_targets_of_lambda_node                          (node.as_lambda_node().unwrap()                          ),
        Node::LocalVariableAndWriteNode         { .. } => comment_targets_of_local_variable_and_write_node        (node.as_local_variable_and_write_node().unwrap()        ),
        Node::LocalVariableOperatorWriteNode    { .. } => comment_targets_of_local_variable_operator_write_node   (node.as_local_variable_operator_write_node().unwrap()   ),
        Node::LocalVariableOrWriteNode          { .. } => comment_targets_of_local_variable_or_write_node         (node.as_local_variable_or_write_node().unwrap()         ),
        Node::LocalVariableReadNode             { .. } => comment_targets_of_local_variable_read_node             (node.as_local_variable_read_node().unwrap()             ),
        Node::LocalVariableTargetNode           { .. } => comment_targets_of_local_variable_target_node           (node.as_local_variable_target_node().unwrap()           ),
        Node::LocalVariableWriteNode            { .. } => comment_targets_of_local_variable_write_node            (node.as_local_variable_write_node().unwrap()            ),
        Node::MatchLastLineNode                 { .. } => comment_targets_of_match_last_line_node                 (node.as_match_last_line_node().unwrap()                 ),
        Node::MatchPredicateNode                { .. } => comment_targets_of_match_predicate_node                 (node.as_match_predicate_node().unwrap()                 ),
        Node::MatchRequiredNode                 { .. } => comment_targets_of_match_required_node                  (node.as_match_required_node().unwrap()                  ),
        Node::MatchWriteNode                    { .. } => comment_targets_of_match_write_node                     (node.as_match_write_node().unwrap()                     ),
        Node::MissingNode                       { .. } => comment_targets_of_missing_node                         (node.as_missing_node().unwrap()                         ),
        Node::ModuleNode                        { .. } => comment_targets_of_module_node                          (node.as_module_node().unwrap()                          ),
        Node::MultiTargetNode                   { .. } => comment_targets_of_multi_target_node                    (node.as_multi_target_node().unwrap()                    ),
        Node::MultiWriteNode                    { .. } => comment_targets_of_multi_write_node                     (node.as_multi_write_node().unwrap()                     ),
        Node::NextNode                          { .. } => comment_targets_of_next_node                            (node.as_next_node().unwrap()                            ),
        Node::NilNode                           { .. } => comment_targets_of_nil_node                             (node.as_nil_node().unwrap()                             ),
        Node::NoKeywordsParameterNode           { .. } => comment_targets_of_no_keywords_parameter_node           (node.as_no_keywords_parameter_node().unwrap()           ),
        Node::NumberedParametersNode            { .. } => comment_targets_of_numbered_parameters_node             (node.as_numbered_parameters_node().unwrap()             ),
        Node::NumberedReferenceReadNode         { .. } => comment_targets_of_numbered_reference_read_node         (node.as_numbered_reference_read_node().unwrap()         ),
        Node::OptionalKeywordParameterNode      { .. } => comment_targets_of_optional_keyword_parameter_node      (node.as_optional_keyword_parameter_node().unwrap()      ),
        Node::OptionalParameterNode             { .. } => comment_targets_of_optional_parameter_node              (node.as_optional_parameter_node().unwrap()              ),
        Node::OrNode                            { .. } => comment_targets_of_or_node                              (node.as_or_node().unwrap()                              ),
        Node::ParametersNode                    { .. } => comment_targets_of_parameters_node                      (node.as_parameters_node().unwrap()                      ),
        Node::ParenthesesNode                   { .. } => comment_targets_of_parentheses_node                     (node.as_parentheses_node().unwrap()                     ),
        Node::PinnedExpressionNode              { .. } => comment_targets_of_pinned_expression_node               (node.as_pinned_expression_node().unwrap()               ),
        Node::PinnedVariableNode                { .. } => comment_targets_of_pinned_variable_node                 (node.as_pinned_variable_node().unwrap()                 ),
        Node::PostExecutionNode                 { .. } => comment_targets_of_post_execution_node                  (node.as_post_execution_node().unwrap()                  ),
        Node::PreExecutionNode                  { .. } => comment_targets_of_pre_execution_node                   (node.as_pre_execution_node().unwrap()                   ),
        Node::ProgramNode                       { .. } => comment_targets_of_program_node                         (node.as_program_node().unwrap()                         ),
        Node::RangeNode                         { .. } => comment_targets_of_range_node                           (node.as_range_node().unwrap()                           ),
        Node::RationalNode                      { .. } => comment_targets_of_rational_node                        (node.as_rational_node().unwrap()                        ),
        Node::RedoNode                          { .. } => comment_targets_of_redo_node                            (node.as_redo_node().unwrap()                            ),
        Node::RegularExpressionNode             { .. } => comment_targets_of_regular_expression_node              (node.as_regular_expression_node().unwrap()              ),
        Node::RequiredKeywordParameterNode      { .. } => comment_targets_of_required_keyword_parameter_node      (node.as_required_keyword_parameter_node().unwrap()      ),
        Node::RequiredParameterNode             { .. } => comment_targets_of_required_parameter_node              (node.as_required_parameter_node().unwrap()              ),
        Node::RescueModifierNode                { .. } => comment_targets_of_rescue_modifier_node                 (node.as_rescue_modifier_node().unwrap()                 ),
        Node::RescueNode                        { .. } => comment_targets_of_rescue_node                          (node.as_rescue_node().unwrap()                          ),
        Node::RestParameterNode                 { .. } => comment_targets_of_rest_parameter_node                  (node.as_rest_parameter_node().unwrap()                  ),
        Node::RetryNode                         { .. } => comment_targets_of_retry_node                           (node.as_retry_node().unwrap()                           ),
        Node::ReturnNode                        { .. } => comment_targets_of_return_node                          (node.as_return_node().unwrap()                          ),
        Node::SelfNode                          { .. } => comment_targets_of_self_node                            (node.as_self_node().unwrap()                            ),
        Node::ShareableConstantNode             { .. } => comment_targets_of_shareable_constant_node              (node.as_shareable_constant_node().unwrap()              ),
        Node::SingletonClassNode                { .. } => comment_targets_of_singleton_class_node                 (node.as_singleton_class_node().unwrap()                 ),
        Node::SourceEncodingNode                { .. } => comment_targets_of_source_encoding_node                 (node.as_source_encoding_node().unwrap()                 ),
        Node::SourceFileNode                    { .. } => comment_targets_of_source_file_node                     (node.as_source_file_node().unwrap()                     ),
        Node::SourceLineNode                    { .. } => comment_targets_of_source_line_node                     (node.as_source_line_node().unwrap()                     ),
        Node::SplatNode                         { .. } => comment_targets_of_splat_node                           (node.as_splat_node().unwrap()                           ),
        Node::StatementsNode                    { .. } => comment_targets_of_statements_node                      (node.as_statements_node().unwrap()                      ),
        Node::StringNode                        { .. } => comment_targets_of_string_node                          (node.as_string_node().unwrap()                          ),
        Node::SuperNode                         { .. } => comment_targets_of_super_node                           (node.as_super_node().unwrap()                           ),
        Node::SymbolNode                        { .. } => comment_targets_of_symbol_node                          (node.as_symbol_node().unwrap()                          ),
        Node::TrueNode                          { .. } => comment_targets_of_true_node                            (node.as_true_node().unwrap()                            ),
        Node::UndefNode                         { .. } => comment_targets_of_undef_node                           (node.as_undef_node().unwrap()                           ),
        Node::UnlessNode                        { .. } => comment_targets_of_unless_node                          (node.as_unless_node().unwrap()                          ),
        Node::UntilNode                         { .. } => comment_targets_of_until_node                           (node.as_until_node().unwrap()                           ),
        Node::WhenNode                          { .. } => comment_targets_of_when_node                            (node.as_when_node().unwrap()                            ),
        Node::WhileNode                         { .. } => comment_targets_of_while_node                           (node.as_while_node().unwrap()                           ),
        Node::XStringNode                       { .. } => comment_targets_of_x_string_node                        (node.as_x_string_node().unwrap()                        ),
        Node::YieldNode                         { .. } => comment_targets_of_yield_node                           (node.as_yield_node().unwrap()                           ),


        /*
        Node::AliasGlobalVariableNode           { .. } => comment_targets_of_alias_global_variable_node           (AliasGlobalVariableNode           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::AliasMethodNode                   { .. } => comment_targets_of_alias_method_node                    (AliasMethodNode                   { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::AlternationPatternNode            { .. } => comment_targets_of_alternation_pattern_node             (AlternationPatternNode            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::AndNode                           { .. } => comment_targets_of_and_node                             (AndNode                           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ArgumentsNode                     { .. } => comment_targets_of_arguments_node                       (ArgumentsNode                     { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ArrayNode                         { .. } => comment_targets_of_array_node                           (ArrayNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ArrayPatternNode                  { .. } => comment_targets_of_array_pattern_node                   (ArrayPatternNode                  { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::AssocNode                         { .. } => comment_targets_of_assoc_node                           (AssocNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::AssocSplatNode                    { .. } => comment_targets_of_assoc_splat_node                     (AssocSplatNode                    { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::BackReferenceReadNode             { .. } => comment_targets_of_back_reference_read_node             (BackReferenceReadNode             { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::BeginNode                         { .. } => comment_targets_of_begin_node                           (BeginNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::BlockArgumentNode                 { .. } => comment_targets_of_block_argument_node                  (BlockArgumentNode                 { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::BlockLocalVariableNode            { .. } => comment_targets_of_block_local_variable_node            (BlockLocalVariableNode            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::BlockNode                         { .. } => comment_targets_of_block_node                           (BlockNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::BlockParameterNode                { .. } => comment_targets_of_block_parameter_node                 (BlockParameterNode                { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::BlockParametersNode               { .. } => comment_targets_of_block_parameters_node                (BlockParametersNode               { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::BreakNode                         { .. } => comment_targets_of_break_node                           (BreakNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::CallAndWriteNode                  { .. } => comment_targets_of_call_and_write_node                  (CallAndWriteNode                  { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::CallNode                          { .. } => comment_targets_of_call_node                            (CallNode                          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::CallOperatorWriteNode             { .. } => comment_targets_of_call_operator_write_node             (CallOperatorWriteNode             { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::CallOrWriteNode                   { .. } => comment_targets_of_call_or_write_node                   (CallOrWriteNode                   { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::CallTargetNode                    { .. } => comment_targets_of_call_target_node                     (CallTargetNode                    { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::CapturePatternNode                { .. } => comment_targets_of_capture_pattern_node                 (CapturePatternNode                { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::CaseMatchNode                     { .. } => comment_targets_of_case_match_node                      (CaseMatchNode                     { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::CaseNode                          { .. } => comment_targets_of_case_node                            (CaseNode                          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ClassNode                         { .. } => comment_targets_of_class_node                           (ClassNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ClassVariableAndWriteNode         { .. } => comment_targets_of_class_variable_and_write_node        (ClassVariableAndWriteNode         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ClassVariableOperatorWriteNode    { .. } => comment_targets_of_class_variable_operator_write_node   (ClassVariableOperatorWriteNode    { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ClassVariableOrWriteNode          { .. } => comment_targets_of_class_variable_or_write_node         (ClassVariableOrWriteNode          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ClassVariableReadNode             { .. } => comment_targets_of_class_variable_read_node             (ClassVariableReadNode             { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ClassVariableTargetNode           { .. } => comment_targets_of_class_variable_target_node           (ClassVariableTargetNode           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ClassVariableWriteNode            { .. } => comment_targets_of_class_variable_write_node            (ClassVariableWriteNode            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ConstantAndWriteNode              { .. } => comment_targets_of_constant_and_write_node              (ConstantAndWriteNode              { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ConstantOperatorWriteNode         { .. } => comment_targets_of_constant_operator_write_node         (ConstantOperatorWriteNode         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ConstantOrWriteNode               { .. } => comment_targets_of_constant_or_write_node               (ConstantOrWriteNode               { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ConstantPathAndWriteNode          { .. } => comment_targets_of_constant_path_and_write_node         (ConstantPathAndWriteNode          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ConstantPathNode                  { .. } => comment_targets_of_constant_path_node                   (ConstantPathNode                  { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ConstantPathOperatorWriteNode     { .. } => comment_targets_of_constant_path_operator_write_node    (ConstantPathOperatorWriteNode     { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ConstantPathOrWriteNode           { .. } => comment_targets_of_constant_path_or_write_node          (ConstantPathOrWriteNode           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ConstantPathTargetNode            { .. } => comment_targets_of_constant_path_target_node            (ConstantPathTargetNode            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ConstantPathWriteNode             { .. } => comment_targets_of_constant_path_write_node             (ConstantPathWriteNode             { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ConstantReadNode                  { .. } => comment_targets_of_constant_read_node                   (ConstantReadNode                  { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ConstantTargetNode                { .. } => comment_targets_of_constant_target_node                 (ConstantTargetNode                { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ConstantWriteNode                 { .. } => comment_targets_of_constant_write_node                  (ConstantWriteNode                 { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::DefNode                           { .. } => comment_targets_of_def_node                             (DefNode                           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::DefinedNode                       { .. } => comment_targets_of_defined_node                         (DefinedNode                       { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ElseNode                          { .. } => comment_targets_of_else_node                            (ElseNode                          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::EmbeddedStatementsNode            { .. } => comment_targets_of_embedded_statements_node             (EmbeddedStatementsNode            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::EmbeddedVariableNode              { .. } => comment_targets_of_embedded_variable_node               (EmbeddedVariableNode              { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::EnsureNode                        { .. } => comment_targets_of_ensure_node                          (EnsureNode                        { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::FalseNode                         { .. } => comment_targets_of_false_node                           (FalseNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::FindPatternNode                   { .. } => comment_targets_of_find_pattern_node                    (FindPatternNode                   { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::FlipFlopNode                      { .. } => comment_targets_of_flip_flop_node                       (FlipFlopNode                      { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::FloatNode                         { .. } => comment_targets_of_float_node                           (FloatNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ForNode                           { .. } => comment_targets_of_for_node                             (ForNode                           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ForwardingArgumentsNode           { .. } => comment_targets_of_forwarding_arguments_node            (ForwardingArgumentsNode           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ForwardingParameterNode           { .. } => comment_targets_of_forwarding_parameter_node            (ForwardingParameterNode           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ForwardingSuperNode               { .. } => comment_targets_of_forwarding_super_node                (ForwardingSuperNode               { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::GlobalVariableAndWriteNode        { .. } => comment_targets_of_global_variable_and_write_node       (GlobalVariableAndWriteNode        { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::GlobalVariableOperatorWriteNode   { .. } => comment_targets_of_global_variable_operator_write_node  (GlobalVariableOperatorWriteNode   { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::GlobalVariableOrWriteNode         { .. } => comment_targets_of_global_variable_or_write_node        (GlobalVariableOrWriteNode         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::GlobalVariableReadNode            { .. } => comment_targets_of_global_variable_read_node            (GlobalVariableReadNode            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::GlobalVariableTargetNode          { .. } => comment_targets_of_global_variable_target_node          (GlobalVariableTargetNode          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::GlobalVariableWriteNode           { .. } => comment_targets_of_global_variable_write_node           (GlobalVariableWriteNode           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::HashNode                          { .. } => comment_targets_of_hash_node                            (HashNode                          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::HashPatternNode                   { .. } => comment_targets_of_hash_pattern_node                    (HashPatternNode                   { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::IfNode                            { .. } => comment_targets_of_if_node                              (IfNode                            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ImaginaryNode                     { .. } => comment_targets_of_imaginary_node                       (ImaginaryNode                     { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ImplicitNode                      { .. } => comment_targets_of_implicit_node                        (ImplicitNode                      { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ImplicitRestNode                  { .. } => comment_targets_of_implicit_rest_node                   (ImplicitRestNode                  { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::InNode                            { .. } => comment_targets_of_in_node                              (InNode                            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::IndexAndWriteNode                 { .. } => comment_targets_of_index_and_write_node                 (IndexAndWriteNode                 { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::IndexOperatorWriteNode            { .. } => comment_targets_of_index_operator_write_node            (IndexOperatorWriteNode            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::IndexOrWriteNode                  { .. } => comment_targets_of_index_or_write_node                  (IndexOrWriteNode                  { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::IndexTargetNode                   { .. } => comment_targets_of_index_target_node                    (IndexTargetNode                   { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::InstanceVariableAndWriteNode      { .. } => comment_targets_of_instance_variable_and_write_node     (InstanceVariableAndWriteNode      { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::InstanceVariableOperatorWriteNode { .. } => comment_targets_of_instance_variable_operator_write_node(InstanceVariableOperatorWriteNode { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::InstanceVariableOrWriteNode       { .. } => comment_targets_of_instance_variable_or_write_node      (InstanceVariableOrWriteNode       { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::InstanceVariableReadNode          { .. } => comment_targets_of_instance_variable_read_node          (InstanceVariableReadNode          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::InstanceVariableTargetNode        { .. } => comment_targets_of_instance_variable_target_node        (InstanceVariableTargetNode        { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::InstanceVariableWriteNode         { .. } => comment_targets_of_instance_variable_write_node         (InstanceVariableWriteNode         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::IntegerNode                       { .. } => comment_targets_of_integer_node                         (IntegerNode                       { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::InterpolatedMatchLastLineNode     { .. } => comment_targets_of_interpolated_match_last_line_node    (InterpolatedMatchLastLineNode     { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::InterpolatedRegularExpressionNode { .. } => comment_targets_of_interpolated_regular_expression_node (InterpolatedRegularExpressionNode { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::InterpolatedStringNode            { .. } => comment_targets_of_interpolated_string_node             (InterpolatedStringNode            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::InterpolatedSymbolNode            { .. } => comment_targets_of_interpolated_symbol_node             (InterpolatedSymbolNode            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::InterpolatedXStringNode           { .. } => comment_targets_of_interpolated_x_string_node           (InterpolatedXStringNode           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ItLocalVariableReadNode           { .. } => comment_targets_of_it_local_variable_read_node          (ItLocalVariableReadNode           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ItParametersNode                  { .. } => comment_targets_of_it_parameters_node                   (ItParametersNode                  { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::KeywordHashNode                   { .. } => comment_targets_of_keyword_hash_node                    (KeywordHashNode                   { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::KeywordRestParameterNode          { .. } => comment_targets_of_keyword_rest_parameter_node          (KeywordRestParameterNode          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::LambdaNode                        { .. } => comment_targets_of_lambda_node                          (LambdaNode                        { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::LocalVariableAndWriteNode         { .. } => comment_targets_of_local_variable_and_write_node        (LocalVariableAndWriteNode         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::LocalVariableOperatorWriteNode    { .. } => comment_targets_of_local_variable_operator_write_node   (LocalVariableOperatorWriteNode    { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::LocalVariableOrWriteNode          { .. } => comment_targets_of_local_variable_or_write_node         (LocalVariableOrWriteNode          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::LocalVariableReadNode             { .. } => comment_targets_of_local_variable_read_node             (LocalVariableReadNode             { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::LocalVariableTargetNode           { .. } => comment_targets_of_local_variable_target_node           (LocalVariableTargetNode           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::LocalVariableWriteNode            { .. } => comment_targets_of_local_variable_write_node            (LocalVariableWriteNode            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::MatchLastLineNode                 { .. } => comment_targets_of_match_last_line_node                 (MatchLastLineNode                 { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::MatchPredicateNode                { .. } => comment_targets_of_match_predicate_node                 (MatchPredicateNode                { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::MatchRequiredNode                 { .. } => comment_targets_of_match_required_node                  (MatchRequiredNode                 { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::MatchWriteNode                    { .. } => comment_targets_of_match_write_node                     (MatchWriteNode                    { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::MissingNode                       { .. } => comment_targets_of_missing_node                         (MissingNode                       { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ModuleNode                        { .. } => comment_targets_of_module_node                          (ModuleNode                        { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::MultiTargetNode                   { .. } => comment_targets_of_multi_target_node                    (MultiTargetNode                   { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::MultiWriteNode                    { .. } => comment_targets_of_multi_write_node                     (MultiWriteNode                    { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::NextNode                          { .. } => comment_targets_of_next_node                            (NextNode                          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::NilNode                           { .. } => comment_targets_of_nil_node                             (NilNode                           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::NoKeywordsParameterNode           { .. } => comment_targets_of_no_keywords_parameter_node           (NoKeywordsParameterNode           { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::NumberedParametersNode            { .. } => comment_targets_of_numbered_parameters_node             (NumberedParametersNode            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::NumberedReferenceReadNode         { .. } => comment_targets_of_numbered_reference_read_node         (NumberedReferenceReadNode         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::OptionalKeywordParameterNode      { .. } => comment_targets_of_optional_keyword_parameter_node      (OptionalKeywordParameterNode      { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::OptionalParameterNode             { .. } => comment_targets_of_optional_parameter_node              (OptionalParameterNode             { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::OrNode                            { .. } => comment_targets_of_or_node                              (OrNode                            { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ParametersNode                    { .. } => comment_targets_of_parameters_node                      (ParametersNode                    { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ParenthesesNode                   { .. } => comment_targets_of_parentheses_node                     (ParenthesesNode                   { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::PinnedExpressionNode              { .. } => comment_targets_of_pinned_expression_node               (PinnedExpressionNode              { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::PinnedVariableNode                { .. } => comment_targets_of_pinned_variable_node                 (PinnedVariableNode                { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::PostExecutionNode                 { .. } => comment_targets_of_post_execution_node                  (PostExecutionNode                 { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::PreExecutionNode                  { .. } => comment_targets_of_pre_execution_node                   (PreExecutionNode                  { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ProgramNode                       { .. } => comment_targets_of_program_node                         (ProgramNode                       { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::RangeNode                         { .. } => comment_targets_of_range_node                           (RangeNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::RationalNode                      { .. } => comment_targets_of_rational_node                        (RationalNode                      { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::RedoNode                          { .. } => comment_targets_of_redo_node                            (RedoNode                          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::RegularExpressionNode             { .. } => comment_targets_of_regular_expression_node              (RegularExpressionNode             { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::RequiredKeywordParameterNode      { .. } => comment_targets_of_required_keyword_parameter_node      (RequiredKeywordParameterNode      { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::RequiredParameterNode             { .. } => comment_targets_of_required_parameter_node              (RequiredParameterNode             { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::RescueModifierNode                { .. } => comment_targets_of_rescue_modifier_node                 (RescueModifierNode                { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::RescueNode                        { .. } => comment_targets_of_rescue_node                          (RescueNode                        { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::RestParameterNode                 { .. } => comment_targets_of_rest_parameter_node                  (RestParameterNode                 { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::RetryNode                         { .. } => comment_targets_of_retry_node                           (RetryNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ReturnNode                        { .. } => comment_targets_of_return_node                          (ReturnNode                        { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::SelfNode                          { .. } => comment_targets_of_self_node                            (SelfNode                          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::ShareableConstantNode             { .. } => comment_targets_of_shareable_constant_node              (ShareableConstantNode             { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::SingletonClassNode                { .. } => comment_targets_of_singleton_class_node                 (SingletonClassNode                { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::SourceEncodingNode                { .. } => comment_targets_of_source_encoding_node                 (SourceEncodingNode                { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::SourceFileNode                    { .. } => comment_targets_of_source_file_node                     (SourceFileNode                    { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::SourceLineNode                    { .. } => comment_targets_of_source_line_node                     (SourceLineNode                    { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::SplatNode                         { .. } => comment_targets_of_splat_node                           (SplatNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::StatementsNode                    { .. } => comment_targets_of_statements_node                      (StatementsNode                    { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::StringNode                        { .. } => comment_targets_of_string_node                          (StringNode                        { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::SuperNode                         { .. } => comment_targets_of_super_node                           (SuperNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::SymbolNode                        { .. } => comment_targets_of_symbol_node                          (SymbolNode                        { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::TrueNode                          { .. } => comment_targets_of_true_node                            (TrueNode                          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::UndefNode                         { .. } => comment_targets_of_undef_node                           (UndefNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::UnlessNode                        { .. } => comment_targets_of_unless_node                          (UnlessNode                        { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::UntilNode                         { .. } => comment_targets_of_until_node                           (UntilNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::WhenNode                          { .. } => comment_targets_of_when_node                            (WhenNode                          { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::WhileNode                         { .. } => comment_targets_of_while_node                           (WhileNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::XStringNode                       { .. } => comment_targets_of_x_string_node                        (XStringNode                       { parser: *parser, pointer: *pointer, marker: *marker }),
        Node::YieldNode                         { .. } => comment_targets_of_yield_node                           (YieldNode                         { parser: *parser, pointer: *pointer, marker: *marker }),
        */
    }
}










pub fn comment_targets_of_alias_global_variable_node<'sh>           (node: AliasGlobalVariableNode<'sh>           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_alias_method_node<'sh>                    (node: AliasMethodNode<'sh>                   ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_alternation_pattern_node<'sh>             (node: AlternationPatternNode<'sh>            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_and_node<'sh>                             (node: AndNode<'sh>                           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_arguments_node<'sh>                       (node: ArgumentsNode<'sh>                     ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_array_node<'sh>                           (node: ArrayNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_array_pattern_node<'sh>                   (node: ArrayPatternNode<'sh>                  ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_assoc_node<'sh>                           (node: AssocNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_assoc_splat_node<'sh>                     (node: AssocSplatNode<'sh>                    ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_back_reference_read_node<'sh>             (node: BackReferenceReadNode<'sh>             ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_begin_node<'sh>                           (node: BeginNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_block_argument_node<'sh>                  (node: BlockArgumentNode<'sh>                 ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_block_local_variable_node<'sh>            (node: BlockLocalVariableNode<'sh>            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_block_node<'sh>                           (node: BlockNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_block_parameter_node<'sh>                 (node: BlockParameterNode<'sh>                ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_block_parameters_node<'sh>                (node: BlockParametersNode<'sh>               ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_break_node<'sh>                           (node: BreakNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_call_and_write_node<'sh>                  (node: CallAndWriteNode<'sh>                  ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_call_node<'sh>                            (node: CallNode<'sh>                          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_call_operator_write_node<'sh>             (node: CallOperatorWriteNode<'sh>             ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_call_or_write_node<'sh>                   (node: CallOrWriteNode<'sh>                   ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_call_target_node<'sh>                     (node: CallTargetNode<'sh>                    ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_capture_pattern_node<'sh>                 (node: CapturePatternNode<'sh>                ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_case_match_node<'sh>                      (node: CaseMatchNode<'sh>                     ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_case_node<'sh>                            (node: CaseNode<'sh>                          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_class_node<'sh>                           (node: ClassNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_class_variable_and_write_node<'sh>        (node: ClassVariableAndWriteNode<'sh>         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_class_variable_operator_write_node<'sh>   (node: ClassVariableOperatorWriteNode<'sh>    ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_class_variable_or_write_node<'sh>         (node: ClassVariableOrWriteNode<'sh>          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_class_variable_read_node<'sh>             (node: ClassVariableReadNode<'sh>             ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_class_variable_target_node<'sh>           (node: ClassVariableTargetNode<'sh>           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_class_variable_write_node<'sh>            (node: ClassVariableWriteNode<'sh>            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_constant_and_write_node<'sh>              (node: ConstantAndWriteNode<'sh>              ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_constant_operator_write_node<'sh>         (node: ConstantOperatorWriteNode<'sh>         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_constant_or_write_node<'sh>               (node: ConstantOrWriteNode<'sh>               ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_constant_path_and_write_node<'sh>         (node: ConstantPathAndWriteNode<'sh>          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_constant_path_node<'sh>                   (node: ConstantPathNode<'sh>                  ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_constant_path_operator_write_node<'sh>    (node: ConstantPathOperatorWriteNode<'sh>     ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_constant_path_or_write_node<'sh>          (node: ConstantPathOrWriteNode<'sh>           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_constant_path_target_node<'sh>            (node: ConstantPathTargetNode<'sh>            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_constant_path_write_node<'sh>             (node: ConstantPathWriteNode<'sh>             ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_constant_read_node<'sh>                   (node: ConstantReadNode<'sh>                  ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_constant_target_node<'sh>                 (node: ConstantTargetNode<'sh>                ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_constant_write_node<'sh>                  (node: ConstantWriteNode<'sh>                 ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_def_node<'sh>                             (node: DefNode<'sh>                           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_defined_node<'sh>                         (node: DefinedNode<'sh>                       ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_else_node<'sh>                            (node: ElseNode<'sh>                          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_embedded_statements_node<'sh>             (node: EmbeddedStatementsNode<'sh>            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_embedded_variable_node<'sh>               (node: EmbeddedVariableNode<'sh>              ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_ensure_node<'sh>                          (node: EnsureNode<'sh>                        ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_false_node<'sh>                           (node: FalseNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_find_pattern_node<'sh>                    (node: FindPatternNode<'sh>                   ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_flip_flop_node<'sh>                       (node: FlipFlopNode<'sh>                      ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_float_node<'sh>                           (node: FloatNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_for_node<'sh>                             (node: ForNode<'sh>                           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_forwarding_arguments_node<'sh>            (node: ForwardingArgumentsNode<'sh>           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_forwarding_parameter_node<'sh>            (node: ForwardingParameterNode<'sh>           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_forwarding_super_node<'sh>                (node: ForwardingSuperNode<'sh>               ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_global_variable_and_write_node<'sh>       (node: GlobalVariableAndWriteNode<'sh>        ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_global_variable_operator_write_node<'sh>  (node: GlobalVariableOperatorWriteNode<'sh>   ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_global_variable_or_write_node<'sh>        (node: GlobalVariableOrWriteNode<'sh>         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_global_variable_read_node<'sh>            (node: GlobalVariableReadNode<'sh>            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_global_variable_target_node<'sh>          (node: GlobalVariableTargetNode<'sh>          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_global_variable_write_node<'sh>           (node: GlobalVariableWriteNode<'sh>           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_hash_node<'sh>                            (node: HashNode<'sh>                          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_hash_pattern_node<'sh>                    (node: HashPatternNode<'sh>                   ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_if_node<'sh>                              (node: IfNode<'sh>                            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_imaginary_node<'sh>                       (node: ImaginaryNode<'sh>                     ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_implicit_node<'sh>                        (node: ImplicitNode<'sh>                      ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_implicit_rest_node<'sh>                   (node: ImplicitRestNode<'sh>                  ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_in_node<'sh>                              (node: InNode<'sh>                            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_index_and_write_node<'sh>                 (node: IndexAndWriteNode<'sh>                 ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_index_operator_write_node<'sh>            (node: IndexOperatorWriteNode<'sh>            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_index_or_write_node<'sh>                  (node: IndexOrWriteNode<'sh>                  ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_index_target_node<'sh>                    (node: IndexTargetNode<'sh>                   ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_instance_variable_and_write_node<'sh>     (node: InstanceVariableAndWriteNode<'sh>      ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_instance_variable_operator_write_node<'sh>(node: InstanceVariableOperatorWriteNode<'sh> ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_instance_variable_or_write_node<'sh>      (node: InstanceVariableOrWriteNode<'sh>       ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_instance_variable_read_node<'sh>          (node: InstanceVariableReadNode<'sh>          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_instance_variable_target_node<'sh>        (node: InstanceVariableTargetNode<'sh>        ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_instance_variable_write_node<'sh>         (node: InstanceVariableWriteNode<'sh>         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_integer_node<'sh>                         (node: IntegerNode<'sh>                       ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_interpolated_match_last_line_node<'sh>    (node: InterpolatedMatchLastLineNode<'sh>     ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_interpolated_regular_expression_node<'sh> (node: InterpolatedRegularExpressionNode<'sh> ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_interpolated_string_node<'sh>             (node: InterpolatedStringNode<'sh>            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_interpolated_symbol_node<'sh>             (node: InterpolatedSymbolNode<'sh>            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_interpolated_x_string_node<'sh>           (node: InterpolatedXStringNode<'sh>           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_it_local_variable_read_node<'sh>          (node: ItLocalVariableReadNode<'sh>           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_it_parameters_node<'sh>                   (node: ItParametersNode<'sh>                  ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_keyword_hash_node<'sh>                    (node: KeywordHashNode<'sh>                   ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_keyword_rest_parameter_node<'sh>          (node: KeywordRestParameterNode<'sh>          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_lambda_node<'sh>                          (node: LambdaNode<'sh>                        ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_local_variable_and_write_node<'sh>        (node: LocalVariableAndWriteNode<'sh>         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_local_variable_operator_write_node<'sh>   (node: LocalVariableOperatorWriteNode<'sh>    ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_local_variable_or_write_node<'sh>         (node: LocalVariableOrWriteNode<'sh>          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_local_variable_read_node<'sh>             (node: LocalVariableReadNode<'sh>             ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_local_variable_target_node<'sh>           (node: LocalVariableTargetNode<'sh>           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_local_variable_write_node<'sh>            (node: LocalVariableWriteNode<'sh>            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_match_last_line_node<'sh>                 (node: MatchLastLineNode<'sh>                 ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_match_predicate_node<'sh>                 (node: MatchPredicateNode<'sh>                ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_match_required_node<'sh>                  (node: MatchRequiredNode<'sh>                 ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_match_write_node<'sh>                     (node: MatchWriteNode<'sh>                    ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_missing_node<'sh>                         (node: MissingNode<'sh>                       ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_module_node<'sh>                          (node: ModuleNode<'sh>                        ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_multi_target_node<'sh>                    (node: MultiTargetNode<'sh>                   ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_multi_write_node<'sh>                     (node: MultiWriteNode<'sh>                    ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_next_node<'sh>                            (node: NextNode<'sh>                          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_nil_node<'sh>                             (node: NilNode<'sh>                           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_no_keywords_parameter_node<'sh>           (node: NoKeywordsParameterNode<'sh>           ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_numbered_parameters_node<'sh>             (node: NumberedParametersNode<'sh>            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_numbered_reference_read_node<'sh>         (node: NumberedReferenceReadNode<'sh>         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_optional_keyword_parameter_node<'sh>      (node: OptionalKeywordParameterNode<'sh>      ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_optional_parameter_node<'sh>              (node: OptionalParameterNode<'sh>             ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_or_node<'sh>                              (node: OrNode<'sh>                            ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_parameters_node<'sh>                      (node: ParametersNode<'sh>                    ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_parentheses_node<'sh>                     (node: ParenthesesNode<'sh>                   ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_pinned_expression_node<'sh>               (node: PinnedExpressionNode<'sh>              ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_pinned_variable_node<'sh>                 (node: PinnedVariableNode<'sh>                ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_post_execution_node<'sh>                  (node: PostExecutionNode<'sh>                 ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_pre_execution_node<'sh>                   (node: PreExecutionNode<'sh>                  ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_program_node<'sh>                         (node: ProgramNode<'sh>                       ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_range_node<'sh>                           (node: RangeNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_rational_node<'sh>                        (node: RationalNode<'sh>                      ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_redo_node<'sh>                            (node: RedoNode<'sh>                          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_regular_expression_node<'sh>              (node: RegularExpressionNode<'sh>             ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_required_keyword_parameter_node<'sh>      (node: RequiredKeywordParameterNode<'sh>      ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_required_parameter_node<'sh>              (node: RequiredParameterNode<'sh>             ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_rescue_modifier_node<'sh>                 (node: RescueModifierNode<'sh>                ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_rescue_node<'sh>                          (node: RescueNode<'sh>                        ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_rest_parameter_node<'sh>                  (node: RestParameterNode<'sh>                 ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_retry_node<'sh>                           (node: RetryNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_return_node<'sh>                          (node: ReturnNode<'sh>                        ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_self_node<'sh>                            (node: SelfNode<'sh>                          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_shareable_constant_node<'sh>              (node: ShareableConstantNode<'sh>             ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_singleton_class_node<'sh>                 (node: SingletonClassNode<'sh>                ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_source_encoding_node<'sh>                 (node: SourceEncodingNode<'sh>                ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_source_file_node<'sh>                     (node: SourceFileNode<'sh>                    ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_source_line_node<'sh>                     (node: SourceLineNode<'sh>                    ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_splat_node<'sh>                           (node: SplatNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_statements_node<'sh>                      (node: StatementsNode<'sh>                    ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_string_node<'sh>                          (node: StringNode<'sh>                        ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_super_node<'sh>                           (node: SuperNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_symbol_node<'sh>                          (node: SymbolNode<'sh>                        ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_true_node<'sh>                            (node: TrueNode<'sh>                          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_undef_node<'sh>                           (node: UndefNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_unless_node<'sh>                          (node: UnlessNode<'sh>                        ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_until_node<'sh>                           (node: UntilNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_when_node<'sh>                            (node: WhenNode<'sh>                          ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_while_node<'sh>                           (node: WhileNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_x_string_node<'sh>                        (node: XStringNode<'sh>                       ) -> Vec<Target<'sh>> { Vec::new() }
pub fn comment_targets_of_yield_node<'sh>                           (node: YieldNode<'sh>                         ) -> Vec<Target<'sh>> { Vec::new() }