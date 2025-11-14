// https://github.com/ruby/prism/blob/main/lib/prism/parse_result/comments.rb
use ruby_prism::*;
use std::collections::HashMap;

use crate::{NodeVariant, node_likes::location};

pub struct AttachedCommentOffsets {
    pub leading: Vec<usize>,
    pub trailing: Vec<usize>,
}

/// trait for attaching comments to targets
trait Attach {
    fn start_offset(&self) -> usize;
    fn end_offset(&self) -> usize;
    fn is_enclosing(&self, comment: &Comment) -> bool;
    fn leading_comment(
        &self,
        comment: &Comment,
        map: &mut HashMap<(usize, usize), AttachedCommentOffsets>,
    );
    fn trailing_comment(
        &self,
        comment: &Comment,
        map: &mut HashMap<(usize, usize), AttachedCommentOffsets>,
    );
}

/// target node for attaching comments
#[derive(Clone, Copy)]
pub struct Target {
    start_offset: usize,
    end_offset: usize,
    is_node: bool,
}
impl Target {
    fn from_node<'sh>(node: &'sh Node<'sh>) -> Self {
        let loc = node.location();
        Self {
            start_offset: loc.start_offset(),
            end_offset: loc.end_offset(),
            is_node: true,
        }
    }
    fn from_location<'sh>(loc: &Location<'sh>) -> Self {
        Self {
            start_offset: loc.start_offset(),
            end_offset: loc.end_offset(),
            is_node: false,
        }
    }
}
impl Attach for Target {
    fn start_offset(&self) -> usize {
        self.start_offset
    }
    fn end_offset(&self) -> usize {
        self.end_offset
    }
    #[rustfmt::skip]
    fn is_enclosing(&self, comment: &Comment) -> bool {
        match self.is_node {
            true => self.start_offset <= comment.location().start_offset() && comment.location().end_offset() <= self.end_offset,
            false => false,
        }
    }
    fn leading_comment(
        &self,
        comment: &Comment,
        map: &mut HashMap<(usize, usize), AttachedCommentOffsets>,
    ) {
        let key = (self.start_offset(), self.end_offset());
        map.entry(key)
            .and_modify(|attached| attached.leading.push(comment.location().start_offset()))
            .or_insert_with(|| AttachedCommentOffsets {
                leading: Vec::from([comment.location().start_offset()]),
                trailing: Vec::new(),
            });
    }
    fn trailing_comment(
        &self,
        comment: &Comment,
        map: &mut HashMap<(usize, usize), AttachedCommentOffsets>,
    ) {
        let key = (self.start_offset(), self.end_offset());
        map.entry(key)
            .and_modify(|attached| attached.trailing.push(comment.location().start_offset()))
            .or_insert_with(|| AttachedCommentOffsets {
                leading: Vec::new(),
                trailing: Vec::from([comment.location().start_offset()]),
            });
    }
}

/// attach comments to nodes and locations
pub fn attach<'sh>(
    parse_result: &'sh ParseResult<'sh>,
    map: &'sh mut HashMap<(usize, usize), AttachedCommentOffsets>,
) {
    let node = parse_result.node();
    let source = parse_result.source();
    for comment in parse_result.comments() {
        let (preceding, enclosing, following) = nearest_targets(&node, &comment);
        if is_trailing(&comment, source) {
            if let Some(preceding) = preceding {
                preceding.trailing_comment(&comment, map);
            } else {
                if let Some(following) = following {
                    following.leading_comment(&comment, map);
                } else if let Some(enclosing) = enclosing {
                    enclosing.leading_comment(&comment, map);
                } else {
                    Target::from_node(&parse_result.node()).leading_comment(&comment, map);
                }
            }
        } else {
            if let Some(following) = following {
                following.leading_comment(&comment, map);
            } else if let Some(preceding) = preceding {
                preceding.trailing_comment(&comment, map);
            } else {
                if let Some(enclosing) = enclosing {
                    enclosing.leading_comment(&comment, map);
                } else {
                    Target::from_node(&parse_result.node()).leading_comment(&comment, map);
                }
            }
        }
    }
}

/// find nearest targets for a comment
fn nearest_targets<'sh>(
    node: &'sh Node<'sh>,
    comment: &'sh Comment<'sh>,
) -> (Option<Target>, Option<Target>, Option<Target>) {
    let comment_start = comment.location().start_offset();
    let comment_end = comment.location().end_offset();
    {
        // collect targets
        let mut targets = Vec::new();
        let (locations, nodes) = comment_targets(&node);
        for location in &locations {
            targets.push(Target::from_location(location));
        }
        for node_item in &nodes {
            targets.push(Target::from_node(node_item));
        }
        // sort targets by start_offset
        targets.sort_by_key(|t| t.start_offset());
        let mut preceding: Option<Target> = None;
        let mut following: Option<Target> = None;
        // binary search
        let mut left = 0;
        let mut right = targets.len();
        while left < right {
            let middle = (left + right) / 2;
            let target = &targets[middle];
            let target_start = target.start_offset();
            let target_end = target.end_offset();
            // enclosing
            if target.is_enclosing(comment) {
                if let Some(enclosing_node) = nodes.iter().find(|node| {
                    node.location().start_offset() == target_start
                        && node.location().end_offset() == target_end
                }) {
                    return nearest_targets(enclosing_node, &comment);
                }
                unreachable!("always recurses");
            }
            // preceding
            if target_end <= comment_start {
                preceding = Some(*target);
                left = middle + 1;
                continue;
            }
            // following
            if comment_end <= target_start {
                following = Some(*target);
                right = middle;
                continue;
            }
            unreachable!("comment location overlaps with a target location");
        }
        (preceding, Some(Target::from_node(&node)), following)
    }
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
fn comment_targets<'sh>(node: &'sh Node<'sh>) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    match node {
        Node::AliasGlobalVariableNode           { .. } => comment_targets_of_alias_global_variable_node           (&node.as_alias_global_variable_node().unwrap()           ),
        Node::AliasMethodNode                   { .. } => comment_targets_of_alias_method_node                    (&node.as_alias_method_node().unwrap()                    ),
        Node::AlternationPatternNode            { .. } => comment_targets_of_alternation_pattern_node             (&node.as_alternation_pattern_node().unwrap()             ),
        Node::AndNode                           { .. } => comment_targets_of_and_node                             (&node.as_and_node().unwrap()                             ),
        Node::ArgumentsNode                     { .. } => comment_targets_of_arguments_node                       (&node.as_arguments_node().unwrap()                       ),
        Node::ArrayNode                         { .. } => comment_targets_of_array_node                           (&node.as_array_node().unwrap()                           ),
        Node::ArrayPatternNode                  { .. } => comment_targets_of_array_pattern_node                   (&node.as_array_pattern_node().unwrap()                   ),
        Node::AssocNode                         { .. } => comment_targets_of_assoc_node                           (&node.as_assoc_node().unwrap()                           ),
        Node::AssocSplatNode                    { .. } => comment_targets_of_assoc_splat_node                     (&node.as_assoc_splat_node().unwrap()                     ),
        Node::BackReferenceReadNode             { .. } => comment_targets_of_back_reference_read_node             (&node.as_back_reference_read_node().unwrap()             ),
        Node::BeginNode                         { .. } => comment_targets_of_begin_node                           (&node.as_begin_node().unwrap()                           ),
        Node::BlockArgumentNode                 { .. } => comment_targets_of_block_argument_node                  (&node.as_block_argument_node().unwrap()                  ),
        Node::BlockLocalVariableNode            { .. } => comment_targets_of_block_local_variable_node            (&node.as_block_local_variable_node().unwrap()            ),
        Node::BlockNode                         { .. } => comment_targets_of_block_node                           (&node.as_block_node().unwrap()                           ),
        Node::BlockParameterNode                { .. } => comment_targets_of_block_parameter_node                 (&node.as_block_parameter_node().unwrap()                 ),
        Node::BlockParametersNode               { .. } => comment_targets_of_block_parameters_node                (&node.as_block_parameters_node().unwrap()                ),
        Node::BreakNode                         { .. } => comment_targets_of_break_node                           (&node.as_break_node().unwrap()                           ),
        Node::CallAndWriteNode                  { .. } => comment_targets_of_call_and_write_node                  (&node.as_call_and_write_node().unwrap()                  ),
        Node::CallNode                          { .. } => comment_targets_of_call_node                            (&node.as_call_node().unwrap()                            ),
        Node::CallOperatorWriteNode             { .. } => comment_targets_of_call_operator_write_node             (&node.as_call_operator_write_node().unwrap()             ),
        Node::CallOrWriteNode                   { .. } => comment_targets_of_call_or_write_node                   (&node.as_call_or_write_node().unwrap()                   ),
        Node::CallTargetNode                    { .. } => comment_targets_of_call_target_node                     (&node.as_call_target_node().unwrap()                     ),
        Node::CapturePatternNode                { .. } => comment_targets_of_capture_pattern_node                 (&node.as_capture_pattern_node().unwrap()                 ),
        Node::CaseMatchNode                     { .. } => comment_targets_of_case_match_node                      (&node.as_case_match_node().unwrap()                      ),
        Node::CaseNode                          { .. } => comment_targets_of_case_node                            (&node.as_case_node().unwrap()                            ),
        Node::ClassNode                         { .. } => comment_targets_of_class_node                           (&node.as_class_node().unwrap()                           ),
        Node::ClassVariableAndWriteNode         { .. } => comment_targets_of_class_variable_and_write_node        (&node.as_class_variable_and_write_node().unwrap()        ),
        Node::ClassVariableOperatorWriteNode    { .. } => comment_targets_of_class_variable_operator_write_node   (&node.as_class_variable_operator_write_node().unwrap()   ),
        Node::ClassVariableOrWriteNode          { .. } => comment_targets_of_class_variable_or_write_node         (&node.as_class_variable_or_write_node().unwrap()         ),
        Node::ClassVariableReadNode             { .. } => comment_targets_of_class_variable_read_node             (&node.as_class_variable_read_node().unwrap()             ),
        Node::ClassVariableTargetNode           { .. } => comment_targets_of_class_variable_target_node           (&node.as_class_variable_target_node().unwrap()           ),
        Node::ClassVariableWriteNode            { .. } => comment_targets_of_class_variable_write_node            (&node.as_class_variable_write_node().unwrap()            ),
        Node::ConstantAndWriteNode              { .. } => comment_targets_of_constant_and_write_node              (&node.as_constant_and_write_node().unwrap()              ),
        Node::ConstantOperatorWriteNode         { .. } => comment_targets_of_constant_operator_write_node         (&node.as_constant_operator_write_node().unwrap()         ),
        Node::ConstantOrWriteNode               { .. } => comment_targets_of_constant_or_write_node               (&node.as_constant_or_write_node().unwrap()               ),
        Node::ConstantPathAndWriteNode          { .. } => comment_targets_of_constant_path_and_write_node         (&node.as_constant_path_and_write_node().unwrap()         ),
        Node::ConstantPathNode                  { .. } => comment_targets_of_constant_path_node                   (&node.as_constant_path_node().unwrap()                   ),
        Node::ConstantPathOperatorWriteNode     { .. } => comment_targets_of_constant_path_operator_write_node    (&node.as_constant_path_operator_write_node().unwrap()    ),
        Node::ConstantPathOrWriteNode           { .. } => comment_targets_of_constant_path_or_write_node          (&node.as_constant_path_or_write_node().unwrap()          ),
        Node::ConstantPathTargetNode            { .. } => comment_targets_of_constant_path_target_node            (&node.as_constant_path_target_node().unwrap()            ),
        Node::ConstantPathWriteNode             { .. } => comment_targets_of_constant_path_write_node             (&node.as_constant_path_write_node().unwrap()             ),
        Node::ConstantReadNode                  { .. } => comment_targets_of_constant_read_node                   (&node.as_constant_read_node().unwrap()                   ),
        Node::ConstantTargetNode                { .. } => comment_targets_of_constant_target_node                 (&node.as_constant_target_node().unwrap()                 ),
        Node::ConstantWriteNode                 { .. } => comment_targets_of_constant_write_node                  (&node.as_constant_write_node().unwrap()                  ),
        Node::DefNode                           { .. } => comment_targets_of_def_node                             (&node.as_def_node().unwrap()                             ),
        Node::DefinedNode                       { .. } => comment_targets_of_defined_node                         (&node.as_defined_node().unwrap()                         ),
        Node::ElseNode                          { .. } => comment_targets_of_else_node                            (&node.as_else_node().unwrap()                            ),
        Node::EmbeddedStatementsNode            { .. } => comment_targets_of_embedded_statements_node             (&node.as_embedded_statements_node().unwrap()             ),
        Node::EmbeddedVariableNode              { .. } => comment_targets_of_embedded_variable_node               (&node.as_embedded_variable_node().unwrap()               ),
        Node::EnsureNode                        { .. } => comment_targets_of_ensure_node                          (&node.as_ensure_node().unwrap()                          ),
        Node::FalseNode                         { .. } => comment_targets_of_false_node                           (&node.as_false_node().unwrap()                           ),
        Node::FindPatternNode                   { .. } => comment_targets_of_find_pattern_node                    (&node.as_find_pattern_node().unwrap()                    ),
        Node::FlipFlopNode                      { .. } => comment_targets_of_flip_flop_node                       (&node.as_flip_flop_node().unwrap()                       ),
        Node::FloatNode                         { .. } => comment_targets_of_float_node                           (&node.as_float_node().unwrap()                           ),
        Node::ForNode                           { .. } => comment_targets_of_for_node                             (&node.as_for_node().unwrap()                             ),
        Node::ForwardingArgumentsNode           { .. } => comment_targets_of_forwarding_arguments_node            (&node.as_forwarding_arguments_node().unwrap()            ),
        Node::ForwardingParameterNode           { .. } => comment_targets_of_forwarding_parameter_node            (&node.as_forwarding_parameter_node().unwrap()            ),
        Node::ForwardingSuperNode               { .. } => comment_targets_of_forwarding_super_node                (&node.as_forwarding_super_node().unwrap()                ),
        Node::GlobalVariableAndWriteNode        { .. } => comment_targets_of_global_variable_and_write_node       (&node.as_global_variable_and_write_node().unwrap()       ),
        Node::GlobalVariableOperatorWriteNode   { .. } => comment_targets_of_global_variable_operator_write_node  (&node.as_global_variable_operator_write_node().unwrap()  ),
        Node::GlobalVariableOrWriteNode         { .. } => comment_targets_of_global_variable_or_write_node        (&node.as_global_variable_or_write_node().unwrap()        ),
        Node::GlobalVariableReadNode            { .. } => comment_targets_of_global_variable_read_node            (&node.as_global_variable_read_node().unwrap()            ),
        Node::GlobalVariableTargetNode          { .. } => comment_targets_of_global_variable_target_node          (&node.as_global_variable_target_node().unwrap()          ),
        Node::GlobalVariableWriteNode           { .. } => comment_targets_of_global_variable_write_node           (&node.as_global_variable_write_node().unwrap()           ),
        Node::HashNode                          { .. } => comment_targets_of_hash_node                            (&node.as_hash_node().unwrap()                            ),
        Node::HashPatternNode                   { .. } => comment_targets_of_hash_pattern_node                    (&node.as_hash_pattern_node().unwrap()                    ),
        Node::IfNode                            { .. } => comment_targets_of_if_node                              (&node.as_if_node().unwrap()                              ),
        Node::ImaginaryNode                     { .. } => comment_targets_of_imaginary_node                       (&node.as_imaginary_node().unwrap()                       ),
        Node::ImplicitNode                      { .. } => comment_targets_of_implicit_node                        (&node.as_implicit_node().unwrap()                        ),
        Node::ImplicitRestNode                  { .. } => comment_targets_of_implicit_rest_node                   (&node.as_implicit_rest_node().unwrap()                   ),
        Node::InNode                            { .. } => comment_targets_of_in_node                              (&node.as_in_node().unwrap()                              ),
        Node::IndexAndWriteNode                 { .. } => comment_targets_of_index_and_write_node                 (&node.as_index_and_write_node().unwrap()                 ),
        Node::IndexOperatorWriteNode            { .. } => comment_targets_of_index_operator_write_node            (&node.as_index_operator_write_node().unwrap()            ),
        Node::IndexOrWriteNode                  { .. } => comment_targets_of_index_or_write_node                  (&node.as_index_or_write_node().unwrap()                  ),
        Node::IndexTargetNode                   { .. } => comment_targets_of_index_target_node                    (&node.as_index_target_node().unwrap()                    ),
        Node::InstanceVariableAndWriteNode      { .. } => comment_targets_of_instance_variable_and_write_node     (&node.as_instance_variable_and_write_node().unwrap()     ),
        Node::InstanceVariableOperatorWriteNode { .. } => comment_targets_of_instance_variable_operator_write_node(&node.as_instance_variable_operator_write_node().unwrap()),
        Node::InstanceVariableOrWriteNode       { .. } => comment_targets_of_instance_variable_or_write_node      (&node.as_instance_variable_or_write_node().unwrap()      ),
        Node::InstanceVariableReadNode          { .. } => comment_targets_of_instance_variable_read_node          (&node.as_instance_variable_read_node().unwrap()          ),
        Node::InstanceVariableTargetNode        { .. } => comment_targets_of_instance_variable_target_node        (&node.as_instance_variable_target_node().unwrap()        ),
        Node::InstanceVariableWriteNode         { .. } => comment_targets_of_instance_variable_write_node         (&node.as_instance_variable_write_node().unwrap()         ),
        Node::IntegerNode                       { .. } => comment_targets_of_integer_node                         (&node.as_integer_node().unwrap()                         ),
        Node::InterpolatedMatchLastLineNode     { .. } => comment_targets_of_interpolated_match_last_line_node    (&node.as_interpolated_match_last_line_node().unwrap()    ),
        Node::InterpolatedRegularExpressionNode { .. } => comment_targets_of_interpolated_regular_expression_node (&node.as_interpolated_regular_expression_node().unwrap() ),
        Node::InterpolatedStringNode            { .. } => comment_targets_of_interpolated_string_node             (&node.as_interpolated_string_node().unwrap()             ),
        Node::InterpolatedSymbolNode            { .. } => comment_targets_of_interpolated_symbol_node             (&node.as_interpolated_symbol_node().unwrap()             ),
        Node::InterpolatedXStringNode           { .. } => comment_targets_of_interpolated_x_string_node           (&node.as_interpolated_x_string_node().unwrap()           ),
        Node::ItLocalVariableReadNode           { .. } => comment_targets_of_it_local_variable_read_node          (&node.as_it_local_variable_read_node().unwrap()          ),
        Node::ItParametersNode                  { .. } => comment_targets_of_it_parameters_node                   (&node.as_it_parameters_node().unwrap()                   ),
        Node::KeywordHashNode                   { .. } => comment_targets_of_keyword_hash_node                    (&node.as_keyword_hash_node().unwrap()                    ),
        Node::KeywordRestParameterNode          { .. } => comment_targets_of_keyword_rest_parameter_node          (&node.as_keyword_rest_parameter_node().unwrap()          ),
        Node::LambdaNode                        { .. } => comment_targets_of_lambda_node                          (&node.as_lambda_node().unwrap()                          ),
        Node::LocalVariableAndWriteNode         { .. } => comment_targets_of_local_variable_and_write_node        (&node.as_local_variable_and_write_node().unwrap()        ),
        Node::LocalVariableOperatorWriteNode    { .. } => comment_targets_of_local_variable_operator_write_node   (&node.as_local_variable_operator_write_node().unwrap()   ),
        Node::LocalVariableOrWriteNode          { .. } => comment_targets_of_local_variable_or_write_node         (&node.as_local_variable_or_write_node().unwrap()         ),
        Node::LocalVariableReadNode             { .. } => comment_targets_of_local_variable_read_node             (&node.as_local_variable_read_node().unwrap()             ),
        Node::LocalVariableTargetNode           { .. } => comment_targets_of_local_variable_target_node           (&node.as_local_variable_target_node().unwrap()           ),
        Node::LocalVariableWriteNode            { .. } => comment_targets_of_local_variable_write_node            (&node.as_local_variable_write_node().unwrap()            ),
        Node::MatchLastLineNode                 { .. } => comment_targets_of_match_last_line_node                 (&node.as_match_last_line_node().unwrap()                 ),
        Node::MatchPredicateNode                { .. } => comment_targets_of_match_predicate_node                 (&node.as_match_predicate_node().unwrap()                 ),
        Node::MatchRequiredNode                 { .. } => comment_targets_of_match_required_node                  (&node.as_match_required_node().unwrap()                  ),
        Node::MatchWriteNode                    { .. } => comment_targets_of_match_write_node                     (&node.as_match_write_node().unwrap()                     ),
        Node::MissingNode                       { .. } => comment_targets_of_missing_node                         (&node.as_missing_node().unwrap()                         ),
        Node::ModuleNode                        { .. } => comment_targets_of_module_node                          (&node.as_module_node().unwrap()                          ),
        Node::MultiTargetNode                   { .. } => comment_targets_of_multi_target_node                    (&node.as_multi_target_node().unwrap()                    ),
        Node::MultiWriteNode                    { .. } => comment_targets_of_multi_write_node                     (&node.as_multi_write_node().unwrap()                     ),
        Node::NextNode                          { .. } => comment_targets_of_next_node                            (&node.as_next_node().unwrap()                            ),
        Node::NilNode                           { .. } => comment_targets_of_nil_node                             (&node.as_nil_node().unwrap()                             ),
        Node::NoKeywordsParameterNode           { .. } => comment_targets_of_no_keywords_parameter_node           (&node.as_no_keywords_parameter_node().unwrap()           ),
        Node::NumberedParametersNode            { .. } => comment_targets_of_numbered_parameters_node             (&node.as_numbered_parameters_node().unwrap()             ),
        Node::NumberedReferenceReadNode         { .. } => comment_targets_of_numbered_reference_read_node         (&node.as_numbered_reference_read_node().unwrap()         ),
        Node::OptionalKeywordParameterNode      { .. } => comment_targets_of_optional_keyword_parameter_node      (&node.as_optional_keyword_parameter_node().unwrap()      ),
        Node::OptionalParameterNode             { .. } => comment_targets_of_optional_parameter_node              (&node.as_optional_parameter_node().unwrap()              ),
        Node::OrNode                            { .. } => comment_targets_of_or_node                              (&node.as_or_node().unwrap()                              ),
        Node::ParametersNode                    { .. } => comment_targets_of_parameters_node                      (&node.as_parameters_node().unwrap()                      ),
        Node::ParenthesesNode                   { .. } => comment_targets_of_parentheses_node                     (&node.as_parentheses_node().unwrap()                     ),
        Node::PinnedExpressionNode              { .. } => comment_targets_of_pinned_expression_node               (&node.as_pinned_expression_node().unwrap()               ),
        Node::PinnedVariableNode                { .. } => comment_targets_of_pinned_variable_node                 (&node.as_pinned_variable_node().unwrap()                 ),
        Node::PostExecutionNode                 { .. } => comment_targets_of_post_execution_node                  (&node.as_post_execution_node().unwrap()                  ),
        Node::PreExecutionNode                  { .. } => comment_targets_of_pre_execution_node                   (&node.as_pre_execution_node().unwrap()                   ),
        Node::ProgramNode                       { .. } => comment_targets_of_program_node                         (&node.as_program_node().unwrap()                         ),
        Node::RangeNode                         { .. } => comment_targets_of_range_node                           (&node.as_range_node().unwrap()                           ),
        Node::RationalNode                      { .. } => comment_targets_of_rational_node                        (&node.as_rational_node().unwrap()                        ),
        Node::RedoNode                          { .. } => comment_targets_of_redo_node                            (&node.as_redo_node().unwrap()                            ),
        Node::RegularExpressionNode             { .. } => comment_targets_of_regular_expression_node              (&node.as_regular_expression_node().unwrap()              ),
        Node::RequiredKeywordParameterNode      { .. } => comment_targets_of_required_keyword_parameter_node      (&node.as_required_keyword_parameter_node().unwrap()      ),
        Node::RequiredParameterNode             { .. } => comment_targets_of_required_parameter_node              (&node.as_required_parameter_node().unwrap()              ),
        Node::RescueModifierNode                { .. } => comment_targets_of_rescue_modifier_node                 (&node.as_rescue_modifier_node().unwrap()                 ),
        Node::RescueNode                        { .. } => comment_targets_of_rescue_node                          (&node.as_rescue_node().unwrap()                          ),
        Node::RestParameterNode                 { .. } => comment_targets_of_rest_parameter_node                  (&node.as_rest_parameter_node().unwrap()                  ),
        Node::RetryNode                         { .. } => comment_targets_of_retry_node                           (&node.as_retry_node().unwrap()                           ),
        Node::ReturnNode                        { .. } => comment_targets_of_return_node                          (&node.as_return_node().unwrap()                          ),
        Node::SelfNode                          { .. } => comment_targets_of_self_node                            (&node.as_self_node().unwrap()                            ),
        Node::ShareableConstantNode             { .. } => comment_targets_of_shareable_constant_node              (&node.as_shareable_constant_node().unwrap()              ),
        Node::SingletonClassNode                { .. } => comment_targets_of_singleton_class_node                 (&node.as_singleton_class_node().unwrap()                 ),
        Node::SourceEncodingNode                { .. } => comment_targets_of_source_encoding_node                 (&node.as_source_encoding_node().unwrap()                 ),
        Node::SourceFileNode                    { .. } => comment_targets_of_source_file_node                     (&node.as_source_file_node().unwrap()                     ),
        Node::SourceLineNode                    { .. } => comment_targets_of_source_line_node                     (&node.as_source_line_node().unwrap()                     ),
        Node::SplatNode                         { .. } => comment_targets_of_splat_node                           (&node.as_splat_node().unwrap()                           ),
        Node::StatementsNode                    { .. } => comment_targets_of_statements_node                      (&node.as_statements_node().unwrap()                      ),
        Node::StringNode                        { .. } => comment_targets_of_string_node                          (&node.as_string_node().unwrap()                          ),
        Node::SuperNode                         { .. } => comment_targets_of_super_node                           (&node.as_super_node().unwrap()                           ),
        Node::SymbolNode                        { .. } => comment_targets_of_symbol_node                          (&node.as_symbol_node().unwrap()                          ),
        Node::TrueNode                          { .. } => comment_targets_of_true_node                            (&node.as_true_node().unwrap()                            ),
        Node::UndefNode                         { .. } => comment_targets_of_undef_node                           (&node.as_undef_node().unwrap()                           ),
        Node::UnlessNode                        { .. } => comment_targets_of_unless_node                          (&node.as_unless_node().unwrap()                          ),
        Node::UntilNode                         { .. } => comment_targets_of_until_node                           (&node.as_until_node().unwrap()                           ),
        Node::WhenNode                          { .. } => comment_targets_of_when_node                            (&node.as_when_node().unwrap()                            ),
        Node::WhileNode                         { .. } => comment_targets_of_while_node                           (&node.as_while_node().unwrap()                           ),
        Node::XStringNode                       { .. } => comment_targets_of_x_string_node                        (&node.as_x_string_node().unwrap()                        ),
        Node::YieldNode                         { .. } => comment_targets_of_yield_node                           (&node.as_yield_node().unwrap()                           ),
    }
}

pub fn push_loc<'sh>(locations: &mut Vec<Location<'sh>>, location: Location<'sh>) {
    locations.push(location);
}
pub fn push_loc_opt<'sh>(locations: &mut Vec<Location<'sh>>, location: Option<Location<'sh>>) {
    if let Some(loc) = location {
        locations.push(loc);
    }
}
pub fn push_node<'sh>(nodes: &mut Vec<Node<'sh>>, node: Node<'sh>) {
    nodes.push(node);
}
pub fn push_node_opt<'sh>(nodes: &mut Vec<Node<'sh>>, node: Option<Node<'sh>>) {
    if let Some(n) = node {
        nodes.push(n);
    }
}
pub fn push_nodelist<'sh>(nodes: &mut Vec<Node<'sh>>, nodelist: NodeList<'sh>) {
    for node in nodelist.iter() {
        nodes.push(node);
    }
}
pub fn push_nodelist_opt<'sh>(nodes: &mut Vec<Node<'sh>>, nodelist: Option<NodeList<'sh>>) {
    if let Some(nl) = nodelist {
        for node in nl.iter() {
            nodes.push(node);
        }
    }
}
pub fn push_statements_opt<'sh>(
    nodes: &mut Vec<Node<'sh>>,
    statements: Option<StatementsNode<'sh>>,
) {
    if let Some(stmts) = statements {
        for node in stmts.body().iter() {
            nodes.push(node);
        }
    }
}
pub fn push_rescue_clauses_opt<'sh>(
    nodes: &mut Vec<Node<'sh>>,
    rescue_clauses: Option<RescueNode<'sh>>,
) {
    if let Some(rescue_clauses) = rescue_clauses {
        nodes.push(rescue_clauses.as_node());
    }
}
pub fn push_else_clause_opt<'sh>(nodes: &mut Vec<Node<'sh>>, else_clause: Option<ElseNode<'sh>>) {
    if let Some(else_clause) = else_clause {
        nodes.push(else_clause.as_node());
    }
}
pub fn push_ensure_clause_opt<'sh>(
    nodes: &mut Vec<Node<'sh>>,
    ensure_clause: Option<EnsureNode<'sh>>,
) {
    if let Some(ensure_clause) = ensure_clause {
        nodes.push(ensure_clause.as_node());
    }
}
pub fn push_parameters_opt<'sh>(
    nodes: &mut Vec<Node<'sh>>,
    parameters: Option<ParametersNode<'sh>>,
) {
    if let Some(params) = parameters {
        nodes.push(params.as_node());
    }
}
pub fn push_arguments_opt<'sh>(nodes: &mut Vec<Node<'sh>>, arguments: Option<ArgumentsNode<'sh>>) {
    if let Some(args) = arguments {
        nodes.push(args.as_node());
    }
}

pub fn comment_targets_of_alias_global_variable_node<'sh>(
    node: &AliasGlobalVariableNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.keyword_loc());
    push_node(&mut nodes, node.new_name());
    push_node(&mut nodes, node.old_name());
    (locations, nodes)
}
pub fn comment_targets_of_alias_method_node<'sh>(
    node: &AliasMethodNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.keyword_loc());
    push_node(&mut nodes, node.new_name());
    push_node(&mut nodes, node.old_name());
    (locations, nodes)
}
pub fn comment_targets_of_alternation_pattern_node<'sh>(
    node: &AlternationPatternNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.left());
    push_node(&mut nodes, node.right());
    (locations, nodes)
}
pub fn comment_targets_of_and_node<'sh>(
    node: &AndNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.left());
    push_node(&mut nodes, node.right());
    (locations, nodes)
}
pub fn comment_targets_of_arguments_node<'sh>(
    node: &ArgumentsNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let locations = Vec::new();
    let mut nodes = Vec::new();
    push_nodelist(&mut nodes, node.arguments());
    (locations, nodes)
}
pub fn comment_targets_of_array_node<'sh>(
    node: &ArrayNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.opening_loc());
    push_loc_opt(&mut locations, node.closing_loc());
    push_nodelist(&mut nodes, node.elements());
    (locations, nodes)
}
pub fn comment_targets_of_array_pattern_node<'sh>(
    node: &ArrayPatternNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.opening_loc());
    push_loc_opt(&mut locations, node.closing_loc());
    push_node_opt(&mut nodes, node.constant());
    push_nodelist(&mut nodes, node.requireds());
    push_node_opt(&mut nodes, node.rest());
    push_nodelist(&mut nodes, node.posts());
    (locations, nodes)
}
pub fn comment_targets_of_assoc_node<'sh>(
    node: &AssocNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.key());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_assoc_splat_node<'sh>(
    node: &AssocSplatNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_node_opt(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_back_reference_read_node<'sh>(
    node: &BackReferenceReadNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_begin_node<'sh>(
    node: &BeginNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.begin_keyword_loc());
    push_loc_opt(&mut locations, node.end_keyword_loc());
    push_statements_opt(&mut nodes, node.statements());
    push_rescue_clauses_opt(&mut nodes, node.rescue_clause());
    push_else_clause_opt(&mut nodes, node.else_clause());
    push_ensure_clause_opt(&mut nodes, node.ensure_clause());
    (locations, nodes)
}
pub fn comment_targets_of_block_argument_node<'sh>(
    node: &BlockArgumentNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_node_opt(&mut nodes, node.expression());
    (locations, nodes)
}
pub fn comment_targets_of_block_local_variable_node<'sh>(
    node: &BlockLocalVariableNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_block_node<'sh>(
    node: &BlockNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.opening_loc());
    push_loc(&mut locations, node.closing_loc());
    push_node_opt(&mut nodes, node.parameters());
    push_node_opt(&mut nodes, node.body());
    (locations, nodes)
}
pub fn comment_targets_of_block_parameter_node<'sh>(
    node: &BlockParameterNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc_opt(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    (locations, nodes)
}
pub fn comment_targets_of_block_parameters_node<'sh>(
    node: &BlockParametersNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.opening_loc());
    push_loc_opt(&mut locations, node.closing_loc());
    push_parameters_opt(&mut nodes, node.parameters());
    push_nodelist(&mut nodes, node.locals());
    (locations, nodes)
}
pub fn comment_targets_of_break_node<'sh>(
    node: &BreakNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.keyword_loc());
    push_arguments_opt(&mut nodes, node.arguments());
    (locations, nodes)
}
pub fn comment_targets_of_call_and_write_node<'sh>(
    node: &CallAndWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.call_operator_loc());
    push_loc_opt(&mut locations, node.message_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node_opt(&mut nodes, node.receiver());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_call_node<'sh>(
    node: &CallNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.call_operator_loc());
    push_loc_opt(&mut locations, node.message_loc());
    push_loc_opt(&mut locations, node.opening_loc());
    push_loc_opt(&mut locations, node.closing_loc());
    push_node_opt(&mut nodes, node.receiver());
    push_arguments_opt(&mut nodes, node.arguments());
    push_node_opt(&mut nodes, node.block());
    (locations, nodes)
}
pub fn comment_targets_of_call_operator_write_node<'sh>(
    node: &CallOperatorWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.call_operator_loc());
    push_loc_opt(&mut locations, node.message_loc());
    push_loc(&mut locations, node.binary_operator_loc());
    push_node_opt(&mut nodes, node.receiver());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_call_or_write_node<'sh>(
    node: &CallOrWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.call_operator_loc());
    push_loc_opt(&mut locations, node.message_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node_opt(&mut nodes, node.receiver());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_call_target_node<'sh>(
    node: &CallTargetNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.call_operator_loc());
    push_loc(&mut locations, node.message_loc());
    push_node(&mut nodes, node.receiver());
    (locations, nodes)
}
pub fn comment_targets_of_capture_pattern_node<'sh>(
    node: &CapturePatternNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    push_node(&mut nodes, node.target().as_node());
    (locations, nodes)
}
pub fn comment_targets_of_case_match_node<'sh>(
    node: &CaseMatchNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.case_keyword_loc());
    push_loc(&mut locations, node.end_keyword_loc());
    push_node_opt(&mut nodes, node.predicate());
    push_nodelist(&mut nodes, node.conditions());
    push_else_clause_opt(&mut nodes, node.else_clause());
    (locations, nodes)
}
pub fn comment_targets_of_case_node<'sh>(
    node: &CaseNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.case_keyword_loc());
    push_loc(&mut locations, node.end_keyword_loc());
    push_node_opt(&mut nodes, node.predicate());
    push_nodelist(&mut nodes, node.conditions());
    push_else_clause_opt(&mut nodes, node.else_clause());
    (locations, nodes)
}
pub fn comment_targets_of_class_node<'sh>(
    node: &ClassNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.class_keyword_loc());
    push_loc_opt(&mut locations, node.inheritance_operator_loc());
    push_loc(&mut locations, node.end_keyword_loc());
    push_node(&mut nodes, node.constant_path());
    push_node_opt(&mut nodes, node.superclass());
    push_node_opt(&mut nodes, node.body());
    (locations, nodes)
}
pub fn comment_targets_of_class_variable_and_write_node<'sh>(
    node: &ClassVariableAndWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_class_variable_operator_write_node<'sh>(
    node: &ClassVariableOperatorWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.binary_operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_class_variable_or_write_node<'sh>(
    node: &ClassVariableOrWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_class_variable_read_node<'sh>(
    node: &ClassVariableReadNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_class_variable_target_node<'sh>(
    node: &ClassVariableTargetNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_class_variable_write_node<'sh>(
    node: &ClassVariableWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_constant_and_write_node<'sh>(
    node: &ConstantAndWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_constant_operator_write_node<'sh>(
    node: &ConstantOperatorWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.binary_operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_constant_or_write_node<'sh>(
    node: &ConstantOrWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_constant_path_and_write_node<'sh>(
    node: &ConstantPathAndWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.target().as_node());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_constant_path_node<'sh>(
    node: &ConstantPathNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.delimiter_loc());
    push_loc(&mut locations, node.name_loc());
    push_node_opt(&mut nodes, node.parent());
    (locations, nodes)
}
pub fn comment_targets_of_constant_path_operator_write_node<'sh>(
    node: &ConstantPathOperatorWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.binary_operator_loc());
    push_node(&mut nodes, node.target().as_node());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_constant_path_or_write_node<'sh>(
    node: &ConstantPathOrWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.target().as_node());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_constant_path_target_node<'sh>(
    node: &ConstantPathTargetNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.delimiter_loc());
    push_loc(&mut locations, node.name_loc());
    push_node_opt(&mut nodes, node.parent());
    (locations, nodes)
}
pub fn comment_targets_of_constant_path_write_node<'sh>(
    node: &ConstantPathWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.target().as_node());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_constant_read_node<'sh>(
    node: &ConstantReadNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_constant_target_node<'sh>(
    node: &ConstantTargetNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_constant_write_node<'sh>(
    node: &ConstantWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_def_node<'sh>(
    node: &DefNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.def_keyword_loc());
    push_loc_opt(&mut locations, node.operator_loc());
    push_loc_opt(&mut locations, node.lparen_loc());
    push_loc_opt(&mut locations, node.rparen_loc());
    push_loc_opt(&mut locations, node.equal_loc());
    push_loc_opt(&mut locations, node.end_keyword_loc());
    push_node_opt(&mut nodes, node.receiver());
    push_parameters_opt(&mut nodes, node.parameters());
    push_node_opt(&mut nodes, node.body());
    (locations, nodes)
}
pub fn comment_targets_of_defined_node<'sh>(
    node: &DefinedNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.lparen_loc());
    push_loc_opt(&mut locations, node.rparen_loc());
    push_loc(&mut locations, node.keyword_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_else_node<'sh>(
    node: &ElseNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.else_keyword_loc());
    push_loc_opt(&mut locations, node.end_keyword_loc());
    push_statements_opt(&mut nodes, node.statements());
    (locations, nodes)
}
pub fn comment_targets_of_embedded_statements_node<'sh>(
    node: &EmbeddedStatementsNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.opening_loc());
    push_loc(&mut locations, node.closing_loc());
    push_statements_opt(&mut nodes, node.statements());
    (locations, nodes)
}
pub fn comment_targets_of_embedded_variable_node<'sh>(
    node: &EmbeddedVariableNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.variable());
    (locations, nodes)
}
pub fn comment_targets_of_ensure_node<'sh>(
    node: &EnsureNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.ensure_keyword_loc());
    push_loc(&mut locations, node.end_keyword_loc());
    push_statements_opt(&mut nodes, node.statements());
    (locations, nodes)
}
pub fn comment_targets_of_false_node<'sh>(
    node: &FalseNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_find_pattern_node<'sh>(
    node: &FindPatternNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.opening_loc());
    push_loc_opt(&mut locations, node.closing_loc());
    push_node_opt(&mut nodes, node.constant());
    push_node(&mut nodes, node.left().as_node());
    push_nodelist(&mut nodes, node.requireds());
    push_node(&mut nodes, node.right());
    (locations, nodes)
}
pub fn comment_targets_of_flip_flop_node<'sh>(
    node: &FlipFlopNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_node_opt(&mut nodes, node.left());
    push_node_opt(&mut nodes, node.right());
    (locations, nodes)
}
pub fn comment_targets_of_float_node<'sh>(
    node: &FloatNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_for_node<'sh>(
    node: &ForNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.for_keyword_loc());
    push_loc(&mut locations, node.in_keyword_loc());
    push_loc_opt(&mut locations, node.do_keyword_loc());
    push_loc(&mut locations, node.end_keyword_loc());
    push_node(&mut nodes, node.index());
    push_node(&mut nodes, node.collection());
    push_statements_opt(&mut nodes, node.statements());
    (locations, nodes)
}
pub fn comment_targets_of_forwarding_arguments_node<'sh>(
    node: &ForwardingArgumentsNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_forwarding_parameter_node<'sh>(
    node: &ForwardingParameterNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_forwarding_super_node<'sh>(
    node: &ForwardingSuperNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let locations = Vec::new();
    let mut nodes = Vec::new();
    if let Some(block) = node.block() {
        push_node(&mut nodes, block.as_node());
    }
    (locations, nodes)
}
pub fn comment_targets_of_global_variable_and_write_node<'sh>(
    node: &GlobalVariableAndWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_global_variable_operator_write_node<'sh>(
    node: &GlobalVariableOperatorWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.binary_operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_global_variable_or_write_node<'sh>(
    node: &GlobalVariableOrWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_global_variable_read_node<'sh>(
    node: &GlobalVariableReadNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_global_variable_target_node<'sh>(
    node: &GlobalVariableTargetNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_global_variable_write_node<'sh>(
    node: &GlobalVariableWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_hash_node<'sh>(
    node: &HashNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.opening_loc());
    push_loc(&mut locations, node.closing_loc());
    push_nodelist(&mut nodes, node.elements());
    (locations, nodes)
}
pub fn comment_targets_of_hash_pattern_node<'sh>(
    node: &HashPatternNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.opening_loc());
    push_loc_opt(&mut locations, node.closing_loc());
    push_node_opt(&mut nodes, node.constant());
    push_nodelist(&mut nodes, node.elements());
    push_node_opt(&mut nodes, node.rest());
    (locations, nodes)
}
pub fn comment_targets_of_if_node<'sh>(node: &IfNode<'sh>) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.if_keyword_loc());
    push_loc_opt(&mut locations, node.then_keyword_loc());
    push_loc_opt(&mut locations, node.end_keyword_loc());
    push_node(&mut nodes, node.predicate());
    push_statements_opt(&mut nodes, node.statements());
    push_node_opt(&mut nodes, node.subsequent());
    (locations, nodes)
}
pub fn comment_targets_of_imaginary_node<'sh>(
    node: &ImaginaryNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_implicit_node<'sh>(
    node: &ImplicitNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let locations = Vec::new();
    let mut nodes = Vec::new();
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_implicit_rest_node<'sh>(
    node: &ImplicitRestNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_in_node<'sh>(node: &InNode<'sh>) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.in_loc());
    push_loc_opt(&mut locations, node.then_loc());
    push_node(&mut nodes, node.pattern());
    push_statements_opt(&mut nodes, node.statements());
    (locations, nodes)
}
pub fn comment_targets_of_index_and_write_node<'sh>(
    node: &IndexAndWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.call_operator_loc());
    push_loc(&mut locations, node.opening_loc());
    push_loc(&mut locations, node.closing_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node_opt(&mut nodes, node.receiver());
    push_arguments_opt(&mut nodes, node.arguments());
    if let Some(block) = node.block() {
        push_node(&mut nodes, block.as_node());
    }
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_index_operator_write_node<'sh>(
    node: &IndexOperatorWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.call_operator_loc());
    push_loc(&mut locations, node.opening_loc());
    push_loc(&mut locations, node.closing_loc());
    push_loc(&mut locations, node.binary_operator_loc());
    push_node_opt(&mut nodes, node.receiver());
    push_arguments_opt(&mut nodes, node.arguments());
    if let Some(block) = node.block() {
        push_node(&mut nodes, block.as_node());
    }
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_index_or_write_node<'sh>(
    node: &IndexOrWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.call_operator_loc());
    push_loc(&mut locations, node.opening_loc());
    push_loc(&mut locations, node.closing_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node_opt(&mut nodes, node.receiver());
    push_arguments_opt(&mut nodes, node.arguments());
    if let Some(block) = node.block() {
        push_node(&mut nodes, block.as_node());
    }
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_index_target_node<'sh>(
    node: &IndexTargetNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.opening_loc());
    push_loc(&mut locations, node.opening_loc());
    push_node(&mut nodes, node.receiver());
    push_arguments_opt(&mut nodes, node.arguments());
    if let Some(block) = node.block() {
        push_node(&mut nodes, block.as_node());
    }
    (locations, nodes)
}
pub fn comment_targets_of_instance_variable_and_write_node<'sh>(
    node: &InstanceVariableAndWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_instance_variable_operator_write_node<'sh>(
    node: &InstanceVariableOperatorWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.binary_operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_instance_variable_or_write_node<'sh>(
    node: &InstanceVariableOrWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_instance_variable_read_node<'sh>(
    node: &InstanceVariableReadNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_instance_variable_target_node<'sh>(
    node: &InstanceVariableTargetNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_instance_variable_write_node<'sh>(
    node: &InstanceVariableWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_integer_node<'sh>(
    node: &IntegerNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_interpolated_match_last_line_node<'sh>(
    node: &InterpolatedMatchLastLineNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.opening_loc());
    push_loc(&mut locations, node.closing_loc());
    push_nodelist(&mut nodes, node.parts());
    (locations, nodes)
}
pub fn comment_targets_of_interpolated_regular_expression_node<'sh>(
    node: &InterpolatedRegularExpressionNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.opening_loc());
    push_loc(&mut locations, node.closing_loc());
    push_nodelist(&mut nodes, node.parts());
    (locations, nodes)
}
pub fn comment_targets_of_interpolated_string_node<'sh>(
    node: &InterpolatedStringNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.opening_loc());
    push_loc_opt(&mut locations, node.closing_loc());
    push_nodelist(&mut nodes, node.parts());
    (locations, nodes)
}
pub fn comment_targets_of_interpolated_symbol_node<'sh>(
    node: &InterpolatedSymbolNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.opening_loc());
    push_loc_opt(&mut locations, node.closing_loc());
    push_nodelist(&mut nodes, node.parts());
    (locations, nodes)
}
pub fn comment_targets_of_interpolated_x_string_node<'sh>(
    node: &InterpolatedXStringNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.opening_loc());
    push_loc(&mut locations, node.closing_loc());
    push_nodelist(&mut nodes, node.parts());
    (locations, nodes)
}
pub fn comment_targets_of_it_local_variable_read_node<'sh>(
    node: &ItLocalVariableReadNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_it_parameters_node<'sh>(
    node: &ItParametersNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_keyword_hash_node<'sh>(
    node: &KeywordHashNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let locations = Vec::new();
    let mut nodes = Vec::new();
    push_nodelist(&mut nodes, node.elements());
    (locations, nodes)
}
pub fn comment_targets_of_keyword_rest_parameter_node<'sh>(
    node: &KeywordRestParameterNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc_opt(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    (locations, nodes)
}
pub fn comment_targets_of_lambda_node<'sh>(
    node: &LambdaNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_loc(&mut locations, node.opening_loc());
    push_loc(&mut locations, node.closing_loc());
    push_node_opt(&mut nodes, node.parameters());
    push_node_opt(&mut nodes, node.body());
    (locations, nodes)
}
pub fn comment_targets_of_local_variable_and_write_node<'sh>(
    node: &LocalVariableAndWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_local_variable_operator_write_node<'sh>(
    node: &LocalVariableOperatorWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.binary_operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_local_variable_or_write_node<'sh>(
    node: &LocalVariableOrWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_local_variable_read_node<'sh>(
    node: &LocalVariableReadNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_local_variable_target_node<'sh>(
    node: &LocalVariableTargetNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_local_variable_write_node<'sh>(
    node: &LocalVariableWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_match_last_line_node<'sh>(
    node: &MatchLastLineNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.opening_loc());
    push_loc(&mut locations, node.content_loc());
    push_loc(&mut locations, node.closing_loc());
    (locations, nodes)
}
pub fn comment_targets_of_match_predicate_node<'sh>(
    node: &MatchPredicateNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    push_node(&mut nodes, node.pattern());
    (locations, nodes)
}
pub fn comment_targets_of_match_required_node<'sh>(
    node: &MatchRequiredNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    push_node(&mut nodes, node.pattern());
    (locations, nodes)
}
pub fn comment_targets_of_match_write_node<'sh>(
    node: &MatchWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let locations = Vec::new();
    let mut nodes = Vec::new();
    push_nodelist(&mut nodes, node.targets());
    (locations, nodes)
}
pub fn comment_targets_of_missing_node<'sh>(
    node: &MissingNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_module_node<'sh>(
    node: &ModuleNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.module_keyword_loc());
    push_loc(&mut locations, node.end_keyword_loc());
    push_node(&mut nodes, node.constant_path());
    push_node_opt(&mut nodes, node.body());
    (locations, nodes)
}
pub fn comment_targets_of_multi_target_node<'sh>(
    node: &MultiTargetNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.lparen_loc());
    push_loc_opt(&mut locations, node.rparen_loc());
    push_nodelist(&mut nodes, node.lefts());
    push_node_opt(&mut nodes, node.rest());
    push_nodelist(&mut nodes, node.rights());
    (locations, nodes)
}
pub fn comment_targets_of_multi_write_node<'sh>(
    node: &MultiWriteNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc_opt(&mut locations, node.lparen_loc());
    push_loc_opt(&mut locations, node.rparen_loc());
    push_loc(&mut locations, node.operator_loc());
    push_nodelist(&mut nodes, node.lefts());
    push_node_opt(&mut nodes, node.rest());
    push_nodelist(&mut nodes, node.rights());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_next_node<'sh>(
    node: &NextNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.keyword_loc());
    push_arguments_opt(&mut nodes, node.arguments());
    (locations, nodes)
}
pub fn comment_targets_of_nil_node<'sh>(
    node: &NilNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_no_keywords_parameter_node<'sh>(
    node: &NoKeywordsParameterNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_loc(&mut locations, node.keyword_loc());
    (locations, nodes)
}
pub fn comment_targets_of_numbered_parameters_node<'sh>(
    node: &NumberedParametersNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_numbered_reference_read_node<'sh>(
    node: &NumberedReferenceReadNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let nodes = Vec::new();
    push_loc(&mut locations, node.location());
    (locations, nodes)
}
pub fn comment_targets_of_optional_keyword_parameter_node<'sh>(
    node: &OptionalKeywordParameterNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_optional_parameter_node<'sh>(
    node: &OptionalParameterNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.name_loc());
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.value());
    (locations, nodes)
}
pub fn comment_targets_of_or_node<'sh>(node: &OrNode<'sh>) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    push_loc(&mut locations, node.operator_loc());
    push_node(&mut nodes, node.left());
    push_node(&mut nodes, node.right());
    (locations, nodes)
}
pub fn comment_targets_of_parameters_node<'sh>(
    node: &ParametersNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_parentheses_node<'sh>(
    node: &ParenthesesNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_pinned_expression_node<'sh>(
    node: &PinnedExpressionNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_pinned_variable_node<'sh>(
    node: &PinnedVariableNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_post_execution_node<'sh>(
    node: &PostExecutionNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_pre_execution_node<'sh>(
    node: &PreExecutionNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_program_node<'sh>(
    node: &ProgramNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_range_node<'sh>(
    node: &RangeNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_rational_node<'sh>(
    node: &RationalNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_redo_node<'sh>(
    node: &RedoNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_regular_expression_node<'sh>(
    node: &RegularExpressionNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_required_keyword_parameter_node<'sh>(
    node: &RequiredKeywordParameterNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_required_parameter_node<'sh>(
    node: &RequiredParameterNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_rescue_modifier_node<'sh>(
    node: &RescueModifierNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_rescue_node<'sh>(
    node: &RescueNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_rest_parameter_node<'sh>(
    node: &RestParameterNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_retry_node<'sh>(
    node: &RetryNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_return_node<'sh>(
    node: &ReturnNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_self_node<'sh>(
    node: &SelfNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_shareable_constant_node<'sh>(
    node: &ShareableConstantNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_singleton_class_node<'sh>(
    node: &SingletonClassNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_source_encoding_node<'sh>(
    node: &SourceEncodingNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_source_file_node<'sh>(
    node: &SourceFileNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_source_line_node<'sh>(
    node: &SourceLineNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_splat_node<'sh>(
    node: &SplatNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_statements_node<'sh>(
    node: &StatementsNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_string_node<'sh>(
    node: &StringNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_super_node<'sh>(
    node: &SuperNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_symbol_node<'sh>(
    node: &SymbolNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_true_node<'sh>(
    node: &TrueNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_undef_node<'sh>(
    node: &UndefNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_unless_node<'sh>(
    node: &UnlessNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_until_node<'sh>(
    node: &UntilNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_when_node<'sh>(
    node: &WhenNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_while_node<'sh>(
    node: &WhileNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_x_string_node<'sh>(
    node: &XStringNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
pub fn comment_targets_of_yield_node<'sh>(
    node: &YieldNode<'sh>,
) -> (Vec<Location<'sh>>, Vec<Node<'sh>>) {
    let mut locations = Vec::new();
    let mut nodes = Vec::new();
    (locations, nodes)
}
