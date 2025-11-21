// https://github.com/ruby/prism/blob/main/lib/prism/parse_result/comments.rb
use ruby_prism::*;
use std::collections::HashMap;

pub struct CommentStore<'sh> {
    by_location: HashMap<(usize, usize), Comment<'sh>>,
    by_target: HashMap<(usize, usize), CommentPlacement>,
}

impl<'sh> CommentStore<'sh> {
    pub fn pop_leading(&mut self, target_start_offset: usize, target_end_offset: usize) -> Option<Vec<Comment<'sh>>> {
        self.by_target
            .get_mut(&(target_start_offset, target_end_offset))
            .and_then(|placement| {
                placement.leading.take().map(|leading_vec| {
                    leading_vec
                        .iter()
                        .filter_map(|(start, end)| self.by_location.remove(&(*start, *end)))
                        .collect::<Vec<Comment>>()
                })
            })
    }
    pub fn pop_trailing(&mut self, target_start_offset: usize, target_end_offset: usize) -> Option<Vec<Comment<'sh>>> {
        self.by_target
            .get_mut(&(target_start_offset, target_end_offset))
            .and_then(|placement| {
                placement.trailing.take().map(|trailing_vec| {
                    trailing_vec
                        .iter()
                        .filter_map(|(start, end)| self.by_location.remove(&(*start, *end)))
                        .collect::<Vec<Comment>>()
                })
            })
    }
    pub fn pop_dangling(&mut self, target_start_offset: usize, target_end_offset: usize) -> Option<Vec<Comment<'sh>>> {
        self.by_target
            .get_mut(&(target_start_offset, target_end_offset))
            .and_then(|placement| {
                placement.dangling.take().map(|dangling_vec| {
                    dangling_vec
                        .iter()
                        .filter_map(|(start, end)| self.by_location.remove(&(*start, *end)))
                        .collect::<Vec<Comment>>()
                })
            })
    }
    pub fn pop_remaining(&mut self, target_start_offset: usize, target_end_offset: usize) -> Option<Vec<Comment<'sh>>> {
        self.by_target
            .get_mut(&(target_start_offset, target_end_offset))
            .and_then(|placement| {
                placement.remaining.take().map(|remaining_vec| {
                    remaining_vec
                        .iter()
                        .filter_map(|(start, end)| self.by_location.remove(&(*start, *end)))
                        .collect::<Vec<Comment>>()
                })
            })
    }
}

/// attached comment offsets for a target
/// - **leading**: comments before the target on preceding lines
/// - **trailing**: comments after the target on the same line
/// - **dangling**: comments after the target on following lines (inside the block)
/// - **remaining**: comments may not be attached to the target (outside the block)
/// ```ruby
///   # leading
///   TARGET_NODE # trailing
///   # dangling
/// # remaining
/// ```
pub struct CommentPlacement {
    pub leading: Option<Vec<(usize, usize)>>,
    pub trailing: Option<Vec<(usize, usize)>>,
    pub dangling: Option<Vec<(usize, usize)>>,
    pub remaining: Option<Vec<(usize, usize)>>,
}

/// trait for attaching comments to targets
trait Attach {
    fn start_offset(&self) -> usize;
    fn end_offset(&self) -> usize;
    fn is_enclosing(&self, comment: &Comment) -> bool;
    fn leading_comment(&self, comment: &Comment, map: &mut HashMap<(usize, usize), CommentPlacement>);
    fn trailing_comment(&self, comment: &Comment, map: &mut HashMap<(usize, usize), CommentPlacement>);
    fn dangling_comment(&self, comment: &Comment, map: &mut HashMap<(usize, usize), CommentPlacement>);
    fn remaining_comment(&self, comment: &Comment, map: &mut HashMap<(usize, usize), CommentPlacement>);
}

/// target node for attaching comments
pub enum Target<'sh> {
    ClosingLocation(Location<'sh>),
    Location(Location<'sh>),
    Node(Node<'sh>),
    OpeningLocation(Location<'sh>),
}
impl<'sh> Target<'sh> {
    pub fn from_node(node: Node<'sh>) -> Self {
        Self::Node(node)
    }
    pub fn from_location(loc: Location<'sh>) -> Self {
        Self::Location(loc)
    }
    pub fn start_offset(&self) -> usize {
        match self {
            Target::ClosingLocation(loc) => loc.start_offset(),
            Target::Location(loc) => loc.start_offset(),
            Target::Node(node) => node.location().start_offset(),
            Target::OpeningLocation(loc) => loc.start_offset(),
        }
    }
    pub fn end_offset(&self) -> usize {
        match self {
            Target::ClosingLocation(loc) => loc.end_offset(),
            Target::Location(loc) => loc.end_offset(),
            Target::Node(node) => node.location().end_offset(),
            Target::OpeningLocation(loc) => loc.end_offset(),
        }
    }
    pub fn location(&self) -> Option<&Location<'sh>> {
        match self {
            Target::Location(loc) => Some(loc),
            Target::ClosingLocation(loc) => Some(loc),
            Target::OpeningLocation(loc) => Some(loc),
            _ => None,
        }
    }
    pub fn node(&self) -> Option<&Node<'sh>> {
        match self {
            Target::Node(node) => Some(node),
            _ => None,
        }
    }
    pub fn is_location(&self) -> bool {
        match self {
            Target::Location(_) => true,
            _ => false,
        }
    }
    pub fn is_node(&self) -> bool {
        match self {
            Target::Node(_) => true,
            _ => false,
        }
    }
}
impl<'sh> Attach for Target<'sh> {
    fn start_offset(&self) -> usize {
        self.start_offset()
    }
    fn end_offset(&self) -> usize {
        self.end_offset()
    }
    #[rustfmt::skip]
    fn is_enclosing(&self, comment: &Comment) -> bool {
        match self.is_node() {
            true => self.start_offset() <= comment.location().start_offset() && comment.location().end_offset() <= self.end_offset(),
            false => false,
        }
    }
    fn leading_comment(&self, comment: &Comment, map: &mut HashMap<(usize, usize), CommentPlacement>) {
        let key = (self.start_offset(), self.end_offset());
        let value = (comment.location().start_offset(), comment.location().end_offset());
        map.entry(key)
            .and_modify(|attached| match &mut attached.leading {
                Some(vec) => vec.push(value),
                None => attached.leading = Some(Vec::from([value])),
            })
            .or_insert_with(|| CommentPlacement {
                leading: Some(Vec::from([value])),
                trailing: None,
                dangling: None,
                remaining: None,
            });
    }
    fn trailing_comment(&self, comment: &Comment, map: &mut HashMap<(usize, usize), CommentPlacement>) {
        let key = (self.start_offset(), self.end_offset());
        let value = (comment.location().start_offset(), comment.location().end_offset());
        map.entry(key)
            .and_modify(|attached| match &mut attached.trailing {
                Some(vec) => vec.push(value),
                None => attached.trailing = Some(Vec::from([value])),
            })
            .or_insert_with(|| CommentPlacement {
                leading: None,
                trailing: Some(Vec::from([value])),
                dangling: None,
                remaining: None,
            });
    }
    fn dangling_comment(&self, comment: &Comment, map: &mut HashMap<(usize, usize), CommentPlacement>) {
        let key = (self.start_offset(), self.end_offset());
        let value = (comment.location().start_offset(), comment.location().end_offset());
        map.entry(key)
            .and_modify(|attached| match &mut attached.dangling {
                Some(vec) => vec.push(value),
                None => attached.dangling = Some(Vec::from([value])),
            })
            .or_insert_with(|| CommentPlacement {
                leading: None,
                trailing: None,
                dangling: Some(Vec::from([value])),
                remaining: None,
            });
    }
    fn remaining_comment(&self, comment: &Comment, map: &mut HashMap<(usize, usize), CommentPlacement>) {
        let key = (self.start_offset(), self.end_offset());
        let value = (comment.location().start_offset(), comment.location().end_offset());
        map.entry(key)
            .and_modify(|attached| match &mut attached.remaining {
                Some(vec) => vec.push(value),
                None => attached.remaining = Some(Vec::from([value])),
            })
            .or_insert_with(|| CommentPlacement {
                leading: None,
                trailing: None,
                dangling: None,
                remaining: Some(Vec::from([value])),
            });
    }
}

/// attach comments to nodes and locations
pub fn attach<'sh>(parse_result: &'sh ParseResult<'sh>) -> CommentStore<'sh> {
    let mut comments_by_location = HashMap::new();
    let mut comments_by_target = HashMap::new();
    let source = parse_result.source();
    for comment in parse_result.comments() {
        let (preceding, enclosing, following) = nearest_targets(Target::Node(parse_result.node()), &comment);
        match is_trailing(&comment, source) {
            true => match (preceding, enclosing, following) {
                (Some(p), _, _) => {
                    p.trailing_comment(&comment, &mut comments_by_target);
                }
                (None, _, Some(f)) => {
                    f.leading_comment(&comment, &mut comments_by_target);
                }
                (None, Some(e), None) => {
                    e.leading_comment(&comment, &mut comments_by_target);
                }
                (None, None, None) => {
                    Target::from_node(parse_result.node()).leading_comment(&comment, &mut comments_by_target);
                }
            },
            false => match (preceding, enclosing, following) {
                (Some(p), _, Some(f)) => match f.is_node() {
                    true => {
                        f.leading_comment(&comment, &mut comments_by_target);
                    }
                    false => {
                        match is_end_keyword(f.start_offset(), f.end_offset(), source) {
                            true => {
                                p.dangling_comment(&comment, &mut comments_by_target);
                            }
                            false => {
                                p.remaining_comment(&comment, &mut comments_by_target);
                            }
                        };
                    }
                },
                (Some(p), _, None) => {
                    p.dangling_comment(&comment, &mut comments_by_target);
                }
                (None, _, Some(f)) => {
                    f.leading_comment(&comment, &mut comments_by_target);
                }
                (None, Some(e), None) => {
                    e.leading_comment(&comment, &mut comments_by_target);
                }
                (None, None, None) => {
                    Target::from_node(parse_result.node()).leading_comment(&comment, &mut comments_by_target);
                }
            },
        }
        comments_by_location.insert(
            (comment.location().start_offset(), comment.location().end_offset()),
            comment,
        );
    }
    CommentStore {
        by_location: comments_by_location,
        by_target: comments_by_target,
    }
}

/// find nearest targets for a comment
fn nearest_targets<'sh>(
    target: Target<'sh>,
    comment: &'sh Comment<'sh>,
) -> (Option<Target<'sh>>, Option<Target<'sh>>, Option<Target<'sh>>) {
    let comment_start = comment.location().start_offset();
    let comment_end = comment.location().end_offset();
    {
        let mut targets = comment_targets(&target);
        targets.sort_by_key(|t| t.start_offset());
        let mut preceding: Option<usize> = None;
        let mut following: Option<usize> = None;
        // binary search
        let mut left = 0;
        let mut right = targets.len();
        while left < right {
            let middle = (left + right) / 2;
            let current_target = &targets[middle];
            let current_target_start = current_target.start_offset();
            let current_target_end = current_target.end_offset();
            // enclosing
            if current_target.is_enclosing(comment) {
                let enclosing_target = targets.swap_remove(middle);
                return nearest_targets(enclosing_target, comment);
            }
            // preceding
            if current_target_end <= comment_start {
                preceding = Some(middle);
                left = middle + 1;
                continue;
            }
            // following
            if comment_end <= current_target_start {
                following = Some(middle);
                right = middle;
                continue;
            }
            unreachable!("comment location overlaps with a target location");
        }

        let following = following.map(|idx| targets.remove(idx));
        let preceding = preceding.map(|idx| targets.remove(idx));
        (preceding, Some(target), following)
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

fn is_end_keyword(start_offset: usize, end_offset: usize, source: &[u8]) -> bool {
    let keyword = &source[start_offset..end_offset];
    keyword == b"end"
}

// #[rustfmt::skip]
// fn comment_targets<'sh>(target: &'sh Target<'sh>) -> Vec<Target<'sh>> {
//     match target {
//         Target::Location(_) => Vec::new(),
//         Target::Node(node) => match target {
//             Node::AliasGlobalVariableNode           { .. } => comment_targets_of_alias_global_variable_node           (&target.as_alias_global_variable_node().unwrap()           ),
//             Node::AliasMethodNode                   { .. } => comment_targets_of_alias_method_node                    (&target.as_alias_method_node().unwrap()                    ),
//             Node::AlternationPatternNode            { .. } => comment_targets_of_alternation_pattern_node             (&target.as_alternation_pattern_node().unwrap()             ),
//             Node::AndNode                           { .. } => comment_targets_of_and_node                             (&target.as_and_node().unwrap()                             ),
//             Node::ArgumentsNode                     { .. } => comment_targets_of_arguments_node                       (&target.as_arguments_node().unwrap()                       ),
//             Node::ArrayNode                         { .. } => comment_targets_of_array_node                           (&target.as_array_node().unwrap()                           ),
//             Node::ArrayPatternNode                  { .. } => comment_targets_of_array_pattern_node                   (&target.as_array_pattern_node().unwrap()                   ),
//             Node::AssocNode                         { .. } => comment_targets_of_assoc_node                           (&target.as_assoc_node().unwrap()                           ),
//             Node::AssocSplatNode                    { .. } => comment_targets_of_assoc_splat_node                     (&target.as_assoc_splat_node().unwrap()                     ),
//             Node::BackReferenceReadNode             { .. } => comment_targets_of_back_reference_read_node             (&target.as_back_reference_read_node().unwrap()             ),
//             Node::BeginNode                         { .. } => comment_targets_of_begin_node                           (&target.as_begin_node().unwrap()                           ),
//             Node::BlockArgumentNode                 { .. } => comment_targets_of_block_argument_node                  (&target.as_block_argument_node().unwrap()                  ),
//             Node::BlockLocalVariableNode            { .. } => comment_targets_of_block_local_variable_node            (&target.as_block_local_variable_node().unwrap()            ),
//             Node::BlockNode                         { .. } => comment_targets_of_block_node                           (&target.as_block_node().unwrap()                           ),
//             Node::BlockParameterNode                { .. } => comment_targets_of_block_parameter_node                 (&target.as_block_parameter_node().unwrap()                 ),
//             Node::BlockParametersNode               { .. } => comment_targets_of_block_parameters_node                (&target.as_block_parameters_node().unwrap()                ),
//             Node::BreakNode                         { .. } => comment_targets_of_break_node                           (&target.as_break_node().unwrap()                           ),
//             Node::CallAndWriteNode                  { .. } => comment_targets_of_call_and_write_node                  (&target.as_call_and_write_node().unwrap()                  ),
//             Node::CallNode                          { .. } => comment_targets_of_call_node                            (&target.as_call_node().unwrap()                            ),
//             Node::CallOperatorWriteNode             { .. } => comment_targets_of_call_operator_write_node             (&target.as_call_operator_write_node().unwrap()             ),
//             Node::CallOrWriteNode                   { .. } => comment_targets_of_call_or_write_node                   (&target.as_call_or_write_node().unwrap()                   ),
//             Node::CallTargetNode                    { .. } => comment_targets_of_call_target_node                     (&target.as_call_target_node().unwrap()                     ),
//             Node::CapturePatternNode                { .. } => comment_targets_of_capture_pattern_node                 (&target.as_capture_pattern_node().unwrap()                 ),
//             Node::CaseMatchNode                     { .. } => comment_targets_of_case_match_node                      (&target.as_case_match_node().unwrap()                      ),
//             Node::CaseNode                          { .. } => comment_targets_of_case_node                            (&target.as_case_node().unwrap()                            ),
//             Node::ClassNode                         { .. } => comment_targets_of_class_node                           (&target.as_class_node().unwrap()                           ),
//             Node::ClassVariableAndWriteNode         { .. } => comment_targets_of_class_variable_and_write_node        (&target.as_class_variable_and_write_node().unwrap()        ),
//             Node::ClassVariableOperatorWriteNode    { .. } => comment_targets_of_class_variable_operator_write_node   (&target.as_class_variable_operator_write_node().unwrap()   ),
//             Node::ClassVariableOrWriteNode          { .. } => comment_targets_of_class_variable_or_write_node         (&target.as_class_variable_or_write_node().unwrap()         ),
//             Node::ClassVariableReadNode             { .. } => comment_targets_of_class_variable_read_node             (&target.as_class_variable_read_node().unwrap()             ),
//             Node::ClassVariableTargetNode           { .. } => comment_targets_of_class_variable_target_node           (&target.as_class_variable_target_node().unwrap()           ),
//             Node::ClassVariableWriteNode            { .. } => comment_targets_of_class_variable_write_node            (&target.as_class_variable_write_node().unwrap()            ),
//             Node::ConstantAndWriteNode              { .. } => comment_targets_of_constant_and_write_node              (&target.as_constant_and_write_node().unwrap()              ),
//             Node::ConstantOperatorWriteNode         { .. } => comment_targets_of_constant_operator_write_node         (&target.as_constant_operator_write_node().unwrap()         ),
//             Node::ConstantOrWriteNode               { .. } => comment_targets_of_constant_or_write_node               (&target.as_constant_or_write_node().unwrap()               ),
//             Node::ConstantPathAndWriteNode          { .. } => comment_targets_of_constant_path_and_write_node         (&target.as_constant_path_and_write_node().unwrap()         ),
//             Node::ConstantPathNode                  { .. } => comment_targets_of_constant_path_node                   (&target.as_constant_path_node().unwrap()                   ),
//             Node::ConstantPathOperatorWriteNode     { .. } => comment_targets_of_constant_path_operator_write_node    (&target.as_constant_path_operator_write_node().unwrap()    ),
//             Node::ConstantPathOrWriteNode           { .. } => comment_targets_of_constant_path_or_write_node          (&target.as_constant_path_or_write_node().unwrap()          ),
//             Node::ConstantPathTargetNode            { .. } => comment_targets_of_constant_path_target_node            (&target.as_constant_path_target_node().unwrap()            ),
//             Node::ConstantPathWriteNode             { .. } => comment_targets_of_constant_path_write_node             (&target.as_constant_path_write_node().unwrap()             ),
//             Node::ConstantReadNode                  { .. } => comment_targets_of_constant_read_node                   (&target.as_constant_read_node().unwrap()                   ),
//             Node::ConstantTargetNode                { .. } => comment_targets_of_constant_target_node                 (&target.as_constant_target_node().unwrap()                 ),
//             Node::ConstantWriteNode                 { .. } => comment_targets_of_constant_write_node                  (&target.as_constant_write_node().unwrap()                  ),
//             Node::DefNode                           { .. } => comment_targets_of_def_node                             (&target.as_def_node().unwrap()                             ),
//             Node::DefinedNode                       { .. } => comment_targets_of_defined_node                         (&target.as_defined_node().unwrap()                         ),
//             Node::ElseNode                          { .. } => comment_targets_of_else_node                            (&target.as_else_node().unwrap()                            ),
//             Node::EmbeddedStatementsNode            { .. } => comment_targets_of_embedded_statements_node             (&target.as_embedded_statements_node().unwrap()             ),
//             Node::EmbeddedVariableNode              { .. } => comment_targets_of_embedded_variable_node               (&target.as_embedded_variable_node().unwrap()               ),
//             Node::EnsureNode                        { .. } => comment_targets_of_ensure_node                          (&target.as_ensure_node().unwrap()                          ),
//             Node::FalseNode                         { .. } => comment_targets_of_false_node                           (&target.as_false_node().unwrap()                           ),
//             Node::FindPatternNode                   { .. } => comment_targets_of_find_pattern_node                    (&target.as_find_pattern_node().unwrap()                    ),
//             Node::FlipFlopNode                      { .. } => comment_targets_of_flip_flop_node                       (&target.as_flip_flop_node().unwrap()                       ),
//             Node::FloatNode                         { .. } => comment_targets_of_float_node                           (&target.as_float_node().unwrap()                           ),
//             Node::ForNode                           { .. } => comment_targets_of_for_node                             (&target.as_for_node().unwrap()                             ),
//             Node::ForwardingArgumentsNode           { .. } => comment_targets_of_forwarding_arguments_node            (&target.as_forwarding_arguments_node().unwrap()            ),
//             Node::ForwardingParameterNode           { .. } => comment_targets_of_forwarding_parameter_node            (&target.as_forwarding_parameter_node().unwrap()            ),
//             Node::ForwardingSuperNode               { .. } => comment_targets_of_forwarding_super_node                (&target.as_forwarding_super_node().unwrap()                ),
//             Node::GlobalVariableAndWriteNode        { .. } => comment_targets_of_global_variable_and_write_node       (&target.as_global_variable_and_write_node().unwrap()       ),
//             Node::GlobalVariableOperatorWriteNode   { .. } => comment_targets_of_global_variable_operator_write_node  (&target.as_global_variable_operator_write_node().unwrap()  ),
//             Node::GlobalVariableOrWriteNode         { .. } => comment_targets_of_global_variable_or_write_node        (&target.as_global_variable_or_write_node().unwrap()        ),
//             Node::GlobalVariableReadNode            { .. } => comment_targets_of_global_variable_read_node            (&target.as_global_variable_read_node().unwrap()            ),
//             Node::GlobalVariableTargetNode          { .. } => comment_targets_of_global_variable_target_node          (&target.as_global_variable_target_node().unwrap()          ),
//             Node::GlobalVariableWriteNode           { .. } => comment_targets_of_global_variable_write_node           (&target.as_global_variable_write_node().unwrap()           ),
//             Node::HashNode                          { .. } => comment_targets_of_hash_node                            (&target.as_hash_node().unwrap()                            ),
//             Node::HashPatternNode                   { .. } => comment_targets_of_hash_pattern_node                    (&target.as_hash_pattern_node().unwrap()                    ),
//             Node::IfNode                            { .. } => comment_targets_of_if_node                              (&target.as_if_node().unwrap()                              ),
//             Node::ImaginaryNode                     { .. } => comment_targets_of_imaginary_node                       (&target.as_imaginary_node().unwrap()                       ),
//             Node::ImplicitNode                      { .. } => comment_targets_of_implicit_node                        (&target.as_implicit_node().unwrap()                        ),
//             Node::ImplicitRestNode                  { .. } => comment_targets_of_implicit_rest_node                   (&target.as_implicit_rest_node().unwrap()                   ),
//             Node::InNode                            { .. } => comment_targets_of_in_node                              (&target.as_in_node().unwrap()                              ),
//             Node::IndexAndWriteNode                 { .. } => comment_targets_of_index_and_write_node                 (&target.as_index_and_write_node().unwrap()                 ),
//             Node::IndexOperatorWriteNode            { .. } => comment_targets_of_index_operator_write_node            (&target.as_index_operator_write_node().unwrap()            ),
//             Node::IndexOrWriteNode                  { .. } => comment_targets_of_index_or_write_node                  (&target.as_index_or_write_node().unwrap()                  ),
//             Node::IndexTargetNode                   { .. } => comment_targets_of_index_target_node                    (&target.as_index_target_node().unwrap()                    ),
//             Node::InstanceVariableAndWriteNode      { .. } => comment_targets_of_instance_variable_and_write_node     (&target.as_instance_variable_and_write_node().unwrap()     ),
//             Node::InstanceVariableOperatorWriteNode { .. } => comment_targets_of_instance_variable_operator_write_node(&target.as_instance_variable_operator_write_node().unwrap()),
//             Node::InstanceVariableOrWriteNode       { .. } => comment_targets_of_instance_variable_or_write_node      (&target.as_instance_variable_or_write_node().unwrap()      ),
//             Node::InstanceVariableReadNode          { .. } => comment_targets_of_instance_variable_read_node          (&target.as_instance_variable_read_node().unwrap()          ),
//             Node::InstanceVariableTargetNode        { .. } => comment_targets_of_instance_variable_target_node        (&target.as_instance_variable_target_node().unwrap()        ),
//             Node::InstanceVariableWriteNode         { .. } => comment_targets_of_instance_variable_write_node         (&target.as_instance_variable_write_node().unwrap()         ),
//             Node::IntegerNode                       { .. } => comment_targets_of_integer_node                         (&target.as_integer_node().unwrap()                         ),
//             Node::InterpolatedMatchLastLineNode     { .. } => comment_targets_of_interpolated_match_last_line_node    (&target.as_interpolated_match_last_line_node().unwrap()    ),
//             Node::InterpolatedRegularExpressionNode { .. } => comment_targets_of_interpolated_regular_expression_node (&target.as_interpolated_regular_expression_node().unwrap() ),
//             Node::InterpolatedStringNode            { .. } => comment_targets_of_interpolated_string_node             (&target.as_interpolated_string_node().unwrap()             ),
//             Node::InterpolatedSymbolNode            { .. } => comment_targets_of_interpolated_symbol_node             (&target.as_interpolated_symbol_node().unwrap()             ),
//             Node::InterpolatedXStringNode           { .. } => comment_targets_of_interpolated_x_string_node           (&target.as_interpolated_x_string_node().unwrap()           ),
//             Node::ItLocalVariableReadNode           { .. } => comment_targets_of_it_local_variable_read_node          (&target.as_it_local_variable_read_node().unwrap()          ),
//             Node::ItParametersNode                  { .. } => comment_targets_of_it_parameters_node                   (&target.as_it_parameters_node().unwrap()                   ),
//             Node::KeywordHashNode                   { .. } => comment_targets_of_keyword_hash_node                    (&target.as_keyword_hash_node().unwrap()                    ),
//             Node::KeywordRestParameterNode          { .. } => comment_targets_of_keyword_rest_parameter_node          (&target.as_keyword_rest_parameter_node().unwrap()          ),
//             Node::LambdaNode                        { .. } => comment_targets_of_lambda_node                          (&target.as_lambda_node().unwrap()                          ),
//             Node::LocalVariableAndWriteNode         { .. } => comment_targets_of_local_variable_and_write_node        (&target.as_local_variable_and_write_node().unwrap()        ),
//             Node::LocalVariableOperatorWriteNode    { .. } => comment_targets_of_local_variable_operator_write_node   (&target.as_local_variable_operator_write_node().unwrap()   ),
//             Node::LocalVariableOrWriteNode          { .. } => comment_targets_of_local_variable_or_write_node         (&target.as_local_variable_or_write_node().unwrap()         ),
//             Node::LocalVariableReadNode             { .. } => comment_targets_of_local_variable_read_node             (&target.as_local_variable_read_node().unwrap()             ),
//             Node::LocalVariableTargetNode           { .. } => comment_targets_of_local_variable_target_node           (&target.as_local_variable_target_node().unwrap()           ),
//             Node::LocalVariableWriteNode            { .. } => comment_targets_of_local_variable_write_node            (&target.as_local_variable_write_node().unwrap()            ),
//             Node::MatchLastLineNode                 { .. } => comment_targets_of_match_last_line_node                 (&target.as_match_last_line_node().unwrap()                 ),
//             Node::MatchPredicateNode                { .. } => comment_targets_of_match_predicate_node                 (&target.as_match_predicate_node().unwrap()                 ),
//             Node::MatchRequiredNode                 { .. } => comment_targets_of_match_required_node                  (&target.as_match_required_node().unwrap()                  ),
//             Node::MatchWriteNode                    { .. } => comment_targets_of_match_write_node                     (&target.as_match_write_node().unwrap()                     ),
//             Node::MissingNode                       { .. } => comment_targets_of_missing_node                         (&target.as_missing_node().unwrap()                         ),
//             Node::ModuleNode                        { .. } => comment_targets_of_module_node                          (&target.as_module_node().unwrap()                          ),
//             Node::MultiTargetNode                   { .. } => comment_targets_of_multi_target_node                    (&target.as_multi_target_node().unwrap()                    ),
//             Node::MultiWriteNode                    { .. } => comment_targets_of_multi_write_node                     (&target.as_multi_write_node().unwrap()                     ),
//             Node::NextNode                          { .. } => comment_targets_of_next_node                            (&target.as_next_node().unwrap()                            ),
//             Node::NilNode                           { .. } => comment_targets_of_nil_node                             (&target.as_nil_node().unwrap()                             ),
//             Node::NoKeywordsParameterNode           { .. } => comment_targets_of_no_keywords_parameter_node           (&target.as_no_keywords_parameter_node().unwrap()           ),
//             Node::NumberedParametersNode            { .. } => comment_targets_of_numbered_parameters_node             (&target.as_numbered_parameters_node().unwrap()             ),
//             Node::NumberedReferenceReadNode         { .. } => comment_targets_of_numbered_reference_read_node         (&target.as_numbered_reference_read_node().unwrap()         ),
//             Node::OptionalKeywordParameterNode      { .. } => comment_targets_of_optional_keyword_parameter_node      (&target.as_optional_keyword_parameter_node().unwrap()      ),
//             Node::OptionalParameterNode             { .. } => comment_targets_of_optional_parameter_node              (&target.as_optional_parameter_node().unwrap()              ),
//             Node::OrNode                            { .. } => comment_targets_of_or_node                              (&target.as_or_node().unwrap()                              ),
//             Node::ParametersNode                    { .. } => comment_targets_of_parameters_node                      (&target.as_parameters_node().unwrap()                      ),
//             Node::ParenthesesNode                   { .. } => comment_targets_of_parentheses_node                     (&target.as_parentheses_node().unwrap()                     ),
//             Node::PinnedExpressionNode              { .. } => comment_targets_of_pinned_expression_node               (&target.as_pinned_expression_node().unwrap()               ),
//             Node::PinnedVariableNode                { .. } => comment_targets_of_pinned_variable_node                 (&target.as_pinned_variable_node().unwrap()                 ),
//             Node::PostExecutionNode                 { .. } => comment_targets_of_post_execution_node                  (&target.as_post_execution_node().unwrap()                  ),
//             Node::PreExecutionNode                  { .. } => comment_targets_of_pre_execution_node                   (&target.as_pre_execution_node().unwrap()                   ),
//             Node::ProgramNode                       { .. } => comment_targets_of_program_node                         (&target.as_program_node().unwrap()                         ),
//             Node::RangeNode                         { .. } => comment_targets_of_range_node                           (&target.as_range_node().unwrap()                           ),
//             Node::RationalNode                      { .. } => comment_targets_of_rational_node                        (&target.as_rational_node().unwrap()                        ),
//             Node::RedoNode                          { .. } => comment_targets_of_redo_node                            (&target.as_redo_node().unwrap()                            ),
//             Node::RegularExpressionNode             { .. } => comment_targets_of_regular_expression_node              (&target.as_regular_expression_node().unwrap()              ),
//             Node::RequiredKeywordParameterNode      { .. } => comment_targets_of_required_keyword_parameter_node      (&target.as_required_keyword_parameter_node().unwrap()      ),
//             Node::RequiredParameterNode             { .. } => comment_targets_of_required_parameter_node              (&target.as_required_parameter_node().unwrap()              ),
//             Node::RescueModifierNode                { .. } => comment_targets_of_rescue_modifier_node                 (&target.as_rescue_modifier_node().unwrap()                 ),
//             Node::RescueNode                        { .. } => comment_targets_of_rescue_node                          (&target.as_rescue_node().unwrap()                          ),
//             Node::RestParameterNode                 { .. } => comment_targets_of_rest_parameter_node                  (&target.as_rest_parameter_node().unwrap()                  ),
//             Node::RetryNode                         { .. } => comment_targets_of_retry_node                           (&target.as_retry_node().unwrap()                           ),
//             Node::ReturnNode                        { .. } => comment_targets_of_return_node                          (&target.as_return_node().unwrap()                          ),
//             Node::SelfNode                          { .. } => comment_targets_of_self_node                            (&target.as_self_node().unwrap()                            ),
//             Node::ShareableConstantNode             { .. } => comment_targets_of_shareable_constant_node              (&target.as_shareable_constant_node().unwrap()              ),
//             Node::SingletonClassNode                { .. } => comment_targets_of_singleton_class_node                 (&target.as_singleton_class_node().unwrap()                 ),
//             Node::SourceEncodingNode                { .. } => comment_targets_of_source_encoding_node                 (&target.as_source_encoding_node().unwrap()                 ),
//             Node::SourceFileNode                    { .. } => comment_targets_of_source_file_node                     (&target.as_source_file_node().unwrap()                     ),
//             Node::SourceLineNode                    { .. } => comment_targets_of_source_line_node                     (&target.as_source_line_node().unwrap()                     ),
//             Node::SplatNode                         { .. } => comment_targets_of_splat_node                           (&target.as_splat_node().unwrap()                           ),
//             Node::StatementsNode                    { .. } => comment_targets_of_statements_node                      (&target.as_statements_node().unwrap()                      ),
//             Node::StringNode                        { .. } => comment_targets_of_string_node                          (&target.as_string_node().unwrap()                          ),
//             Node::SuperNode                         { .. } => comment_targets_of_super_node                           (&target.as_super_node().unwrap()                           ),
//             Node::SymbolNode                        { .. } => comment_targets_of_symbol_node                          (&target.as_symbol_node().unwrap()                          ),
//             Node::TrueNode                          { .. } => comment_targets_of_true_node                            (&target.as_true_node().unwrap()                            ),
//             Node::UndefNode                         { .. } => comment_targets_of_undef_node                           (&target.as_undef_node().unwrap()                           ),
//             Node::UnlessNode                        { .. } => comment_targets_of_unless_node                          (&target.as_unless_node().unwrap()                          ),
//             Node::UntilNode                         { .. } => comment_targets_of_until_node                           (&target.as_until_node().unwrap()                           ),
//             Node::WhenNode                          { .. } => comment_targets_of_when_node                            (&target.as_when_node().unwrap()                            ),
//             Node::WhileNode                         { .. } => comment_targets_of_while_node                           (&target.as_while_node().unwrap()                           ),
//             Node::XStringNode                       { .. } => comment_targets_of_x_string_node                        (&target.as_x_string_node().unwrap()                        ),
//             Node::YieldNode                         { .. } => comment_targets_of_yield_node                           (&target.as_yield_node().unwrap()                           ),
//         }
//     }
// }

#[rustfmt::skip]
fn push_loc_closing<'sh>(loc: Option<Location<'sh>>, targets: &mut Vec<Target<'sh>>) {
    match loc {
        Some(loc) => { targets.push(Target::ClosingLocation(loc)); }
        None => {}
    }
}
#[rustfmt::skip]
fn push_loca<'sh>(loc: Option<Location<'sh>>, targets: &mut Vec<Target<'sh>>) {
    match loc {
        Some(loc) => { targets.push(Target::Location(loc)); }
        None => {}
    }
}
#[rustfmt::skip]
fn push_node<'sh>(node: Option<Node<'sh>>, targets: &mut Vec<Target<'sh>>) {
    match node {
        Some(node) => { targets.push(Target::Node(node)); }
        None => {}
    }
}
#[rustfmt::skip]
fn push_loc_opening<'sh>(loc: Option<Location<'sh>>, targets: &mut Vec<Target<'sh>>) {
    match loc {
        Some(loc) => { targets.push(Target::OpeningLocation(loc)); }
        None => {}
    }
}

#[rustfmt::skip]
fn push_nodelist<'sh>(nodelist: Option<NodeList<'sh>>, targets: &mut Vec<Target<'sh>>) {
    match nodelist {
        Some(nodelist) => {
            for node in nodelist.iter() {
                targets.push(Target::Node(node));
            }
        }
        None => {}
    }
}

pub fn comment_targets_of_alias_global_variable_node<'sh>(node: &AliasGlobalVariableNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.new_name()), &mut targets);
    push_node(Some(node.old_name()), &mut targets);
    push_loca(Some(node.keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_alias_method_node<'sh>(node: &AliasMethodNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.new_name()), &mut targets);
    push_node(Some(node.old_name()), &mut targets);
    push_loca(Some(node.keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_alternation_pattern_node<'sh>(node: &AlternationPatternNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.left()), &mut targets);
    push_node(Some(node.right()), &mut targets);
    push_loca(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_and_node<'sh>(node: &AndNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.left()), &mut targets);
    push_node(Some(node.right()), &mut targets);
    push_loca(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_arguments_node<'sh>(node: &ArgumentsNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_nodelist(Some(node.arguments()), &mut targets);
    targets
}
pub fn comment_targets_of_array_node<'sh>(node: &ArrayNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_nodelist(Some(node.elements()), &mut targets);
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_array_pattern_node<'sh>(node: &ArrayPatternNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.constant(), &mut targets);
    push_nodelist(Some(node.requireds()), &mut targets);
    push_node(node.rest(), &mut targets);
    push_nodelist(Some(node.posts()), &mut targets);
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_assoc_node<'sh>(node: &AssocNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.key()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    push_loca(node.operator_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_assoc_splat_node<'sh>(node: &AssocSplatNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.value(), &mut targets);
    push_loca(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_back_reference_read_node<'sh>(node: &BackReferenceReadNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loca(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_begin_node<'sh>(node: &BeginNode<'sh>) -> Vec<Target<'sh>> {
    fn rescue_clause_recursively<'sh>(rescue_node: &Option<RescueNode<'sh>>, targets: &mut Vec<Target<'sh>>) {
        match rescue_node {
            None => {}
            Some(rescue_node) => {
                push_loca(Some(rescue_node.keyword_loc()), targets);
                push_nodelist(Some(rescue_node.exceptions()), targets);
                push_loca(rescue_node.operator_loc(), targets);
                push_node(rescue_node.reference(), targets);
                push_loca(rescue_node.then_keyword_loc(), targets);
                push_node(rescue_node.statements().map(|stmts| stmts.as_node()), targets);
                rescue_clause_recursively(&rescue_node.subsequent(), targets);
            }
        }
    }
    fn else_clause<'sh>(else_node: &Option<ElseNode<'sh>>, targets: &mut Vec<Target<'sh>>) {
        match else_node {
            None => {}
            Some(else_node) => {
                push_loc_opening(Some(else_node.else_keyword_loc()), targets);
                push_loca(else_node.end_keyword_loc(), targets);
                push_node(else_node.statements().map(|stmts| stmts.as_node()), targets);
            }
        }
    }
    fn ensure_clause<'sh>(ensure_node: &Option<EnsureNode<'sh>>, targets: &mut Vec<Target<'sh>>) {
        match ensure_node {
            None => {}
            Some(ensure_node) => {
                push_loc_opening(Some(ensure_node.ensure_keyword_loc()), targets);
                push_node(ensure_node.statements().map(|stmts| stmts.as_node()), targets);
                push_loc_closing(Some(ensure_node.end_keyword_loc()), targets);
            }
        }
    }
    let mut targets = Vec::new();
    push_loca(node.begin_keyword_loc(), &mut targets);
    push_node(node.statements().map(|stmts| stmts.as_node()), &mut targets);
    rescue_clause_recursively(&node.rescue_clause(), &mut targets);
    else_clause(&node.else_clause(), &mut targets);
    ensure_clause(&node.ensure_clause(), &mut targets);
    push_loca(node.end_keyword_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_block_argument_node<'sh>(node: &BlockArgumentNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loca(Some(node.operator_loc()), &mut targets);
    push_node(node.expression(), &mut targets);
    targets
}
pub fn comment_targets_of_block_local_variable_node<'sh>(node: &BlockLocalVariableNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    targets.push(Target::Location(node.location()));
    targets
}
pub fn comment_targets_of_block_node<'sh>(node: &BlockNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    targets.push(Target::OpeningLocation(node.opening_loc()));
    targets.push(Target::ClosingLocation(node.closing_loc()));
    node.parameters().map(|node| targets.push(Target::Node(node)));
    node.body().map(|node| targets.push(Target::Node(node)));
    targets
}
pub fn comment_targets_of_block_parameter_node<'sh>(node: &BlockParameterNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc().map(|loc| targets.push(Target::Location(loc)));
    targets.push(Target::Location(node.operator_loc()));
    targets
}
pub fn comment_targets_of_block_parameters_node<'sh>(node: &BlockParametersNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc().map(|loc| targets.push(Target::OpeningLocation(loc)));
    node.closing_loc().map(|loc| targets.push(Target::ClosingLocation(loc)));
    node.parameters().map(|node| targets.push(Target::Node(node.as_node())));
    for local in node.locals().iter() {
        targets.push(Target::Node(local));
    }
    targets
}
pub fn comment_targets_of_break_node<'sh>(node: &BreakNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    targets.push(Target::Location(node.keyword_loc()));
    node.arguments().map(|node| targets.push(Target::Node(node.as_node())));
    targets
}
pub fn comment_targets_of_call_and_write_node<'sh>(node: &CallAndWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.call_operator_loc().map(|loc| targets.push(Target::Location(loc)));
    node.message_loc().map(|loc| targets.push(Target::Location(loc)));
    targets.push(Target::Location(node.operator_loc()));
    node.receiver().map(|node| targets.push(Target::Node(node)));
    targets.push(Target::Node(node.value()));
    targets
}
pub fn comment_targets_of_call_node<'sh>(node: &CallNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.receiver().map(|node| targets.push(Target::Node(node)));
    node.call_operator_loc().map(|loc| targets.push(Target::Location(loc)));
    node.message_loc().map(|loc| targets.push(Target::Location(loc)));
    node.opening_loc().map(|loc| targets.push(Target::OpeningLocation(loc)));
    node.arguments().map(|node| targets.push(Target::Node(node.as_node())));
    node.closing_loc().map(|loc| targets.push(Target::ClosingLocation(loc)));
    node.block().map(|node| targets.push(Target::Node(node)));
    targets
}
pub fn comment_targets_of_call_operator_write_node<'sh>(node: &CallOperatorWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.receiver().map(|node| targets.push(Target::Node(node)));
    node.call_operator_loc().map(|loc| targets.push(Target::Location(loc)));
    node.message_loc().map(|loc| targets.push(Target::Location(loc)));
    targets.push(Target::Location(node.binary_operator_loc()));
    targets.push(Target::Node(node.value()));
    targets
}
pub fn comment_targets_of_call_or_write_node<'sh>(node: &CallOrWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.receiver().map(|node| targets.push(Target::Node(node)));
    node.call_operator_loc().map(|loc| targets.push(Target::Location(loc)));
    node.message_loc().map(|loc| targets.push(Target::Location(loc)));
    targets.push(Target::Location(node.operator_loc()));
    targets.push(Target::Node(node.value()));
    targets
}
pub fn comment_targets_of_call_target_node<'sh>(node: &CallTargetNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    targets.push(Target::Node(node.receiver()));
    targets.push(Target::Location(node.call_operator_loc()));
    targets.push(Target::Location(node.message_loc()));
    targets
}
pub fn comment_targets_of_capture_pattern_node<'sh>(node: &CapturePatternNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    targets.push(Target::Node(node.value()));
    targets.push(Target::Node(node.target().as_node()));
    targets.push(Target::Location(node.operator_loc()));
    targets
}
pub fn comment_targets_of_case_match_node<'sh>(node: &CaseMatchNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.predicate().map(|node| targets.push(Target::Node(node)));
    for condition in node.conditions().iter() {
        targets.push(Target::Node(condition));
    }
    node.else_clause()
        .map(|else_clause| targets.push(Target::Node(else_clause.as_node())));
    targets.push(Target::OpeningLocation(node.case_keyword_loc()));
    targets.push(Target::ClosingLocation(node.end_keyword_loc()));
    targets
}
pub fn comment_targets_of_case_node<'sh>(node: &CaseNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.predicate().map(|node| targets.push(Target::Node(node)));
    for condition in node.conditions().iter() {
        targets.push(Target::Node(condition));
    }
    node.else_clause()
        .map(|else_clause| targets.push(Target::Node(else_clause.as_node())));
    targets.push(Target::OpeningLocation(node.case_keyword_loc()));
    targets.push(Target::ClosingLocation(node.end_keyword_loc()));
    targets
}
pub fn comment_targets_of_class_node<'sh>(node: &ClassNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    targets.push(Target::OpeningLocation(node.class_keyword_loc()));
    targets.push(Target::Node(node.constant_path()));
    node.inheritance_operator_loc()
        .map(|loc| targets.push(Target::Location(loc)));
    node.superclass().map(|node| targets.push(Target::Node(node)));
    node.body().map(|node| targets.push(Target::Node(node)));
    targets.push(Target::ClosingLocation(node.end_keyword_loc()));
    targets
}
pub fn comment_targets_of_class_variable_and_write_node<'sh>(
    node: &ClassVariableAndWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    targets.push(Target::Location(node.name_loc()));
    targets.push(Target::Location(node.operator_loc()));
    targets.push(Target::Node(node.value()));
    targets
}
pub fn comment_targets_of_class_variable_operator_write_node<'sh>(
    node: &ClassVariableOperatorWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    targets.push(Target::Location(node.name_loc()));
    targets.push(Target::Location(node.binary_operator_loc()));
    targets.push(Target::Node(node.value()));
    targets
}
pub fn comment_targets_of_class_variable_or_write_node<'sh>(node: &ClassVariableOrWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    targets.push(Target::Location(node.name_loc()));
    targets.push(Target::Location(node.operator_loc()));
    targets.push(Target::Node(node.value()));
    targets
}
pub fn comment_targets_of_class_variable_read_node<'sh>(node: &ClassVariableReadNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    targets.push(Target::Location(node.location()));
    targets
}
pub fn comment_targets_of_class_variable_target_node<'sh>(node: &ClassVariableTargetNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    targets.push(Target::Location(node.location()));
    targets
}
pub fn comment_targets_of_class_variable_write_node<'sh>(node: &ClassVariableWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    targets.push(Target::Location(node.name_loc()));
    targets.push(Target::Node(node.value()));
    targets.push(Target::Location(node.operator_loc()));
    targets
}
/*
pub fn comment_targets_of_constant_and_write_node<'sh>(
    node: &ConstantAndWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_constant_operator_write_node<'sh>(
    node: &ConstantOperatorWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.binary_operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_constant_or_write_node<'sh>(
    node: &ConstantOrWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_constant_path_and_write_node<'sh>(
    node: &ConstantPathAndWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.target().as_node());
    node.value());
    targets
}
pub fn comment_targets_of_constant_path_node<'sh>(
    node: &ConstantPathNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.delimiter_loc());
    node.name_loc());
    node.parent());
    targets
}
pub fn comment_targets_of_constant_path_operator_write_node<'sh>(
    node: &ConstantPathOperatorWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.binary_operator_loc());
    node.target().as_node());
    node.value());
    targets
}
pub fn comment_targets_of_constant_path_or_write_node<'sh>(
    node: &ConstantPathOrWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.target().as_node());
    node.value());
    targets
}
pub fn comment_targets_of_constant_path_target_node<'sh>(
    node: &ConstantPathTargetNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.delimiter_loc());
    node.name_loc());
    node.parent());
    targets
}
pub fn comment_targets_of_constant_path_write_node<'sh>(
    node: &ConstantPathWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.target().as_node());
    node.value());
    targets
}
pub fn comment_targets_of_constant_read_node<'sh>(
    node: &ConstantReadNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_constant_target_node<'sh>(
    node: &ConstantTargetNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_constant_write_node<'sh>(
    node: &ConstantWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_def_node<'sh>(node: &DefNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.def_keyword_loc());
    node.operator_loc());
    node.lparen_loc());
    node.rparen_loc());
    node.equal_loc());
    node.end_keyword_loc());
    node.receiver());
    push_parameters_opt(&mut nodes, node.parameters());
    node.body());
    targets
}
pub fn comment_targets_of_defined_node<'sh>(node: &DefinedNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.lparen_loc());
    node.rparen_loc());
    node.keyword_loc());
    node.value());
    targets
}
pub fn comment_targets_of_else_node<'sh>(node: &ElseNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.else_keyword_loc());
    node.end_keyword_loc());
    push_statements_opt(&mut nodes, node.statements());
    targets
}
pub fn comment_targets_of_embedded_statements_node<'sh>(
    node: &EmbeddedStatementsNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.closing_loc());
    push_statements_opt(&mut nodes, node.statements());
    targets
}
pub fn comment_targets_of_embedded_variable_node<'sh>(
    node: &EmbeddedVariableNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.variable());
    targets
}
pub fn comment_targets_of_ensure_node<'sh>(node: &EnsureNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.ensure_keyword_loc());
    node.end_keyword_loc());
    push_statements_opt(&mut nodes, node.statements());
    targets
}
pub fn comment_targets_of_false_node<'sh>(node: &FalseNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_find_pattern_node<'sh>(node: &FindPatternNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.closing_loc());
    node.constant());
    node.left().as_node());
    node.requireds());
    node.right());
    targets
}
pub fn comment_targets_of_flip_flop_node<'sh>(node: &FlipFlopNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.left());
    node.right());
    targets
}
pub fn comment_targets_of_float_node<'sh>(node: &FloatNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_for_node<'sh>(node: &ForNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.for_keyword_loc());
    node.in_keyword_loc());
    node.do_keyword_loc());
    node.end_keyword_loc());
    node.index());
    node.collection());
    push_statements_opt(&mut nodes, node.statements());
    targets
}
pub fn comment_targets_of_forwarding_arguments_node<'sh>(
    node: &ForwardingArgumentsNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_forwarding_parameter_node<'sh>(
    node: &ForwardingParameterNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_forwarding_super_node<'sh>(
    node: &ForwardingSuperNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    if let Some(block) = node.block() {
        block.as_node());
    }
    targets
}
pub fn comment_targets_of_global_variable_and_write_node<'sh>(
    node: &GlobalVariableAndWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_global_variable_operator_write_node<'sh>(
    node: &GlobalVariableOperatorWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.binary_operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_global_variable_or_write_node<'sh>(
    node: &GlobalVariableOrWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_global_variable_read_node<'sh>(
    node: &GlobalVariableReadNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_global_variable_target_node<'sh>(
    node: &GlobalVariableTargetNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_global_variable_write_node<'sh>(
    node: &GlobalVariableWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_hash_node<'sh>(node: &HashNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.closing_loc());
    node.elements());
    targets
}
pub fn comment_targets_of_hash_pattern_node<'sh>(node: &HashPatternNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.closing_loc());
    node.constant());
    node.elements());
    node.rest());
    targets
}
pub fn comment_targets_of_if_node<'sh>(node: &IfNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.if_keyword_loc());
    node.then_keyword_loc());
    node.end_keyword_loc());
    node.predicate());
    push_statements_opt(&mut nodes, node.statements());
    node.subsequent());
    targets
}
pub fn comment_targets_of_imaginary_node<'sh>(node: &ImaginaryNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_implicit_node<'sh>(node: &ImplicitNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.value());
    targets
}
pub fn comment_targets_of_implicit_rest_node<'sh>(
    node: &ImplicitRestNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_in_node<'sh>(node: &InNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.in_loc());
    node.then_loc());
    node.pattern());
    push_statements_opt(&mut nodes, node.statements());
    targets
}
pub fn comment_targets_of_index_and_write_node<'sh>(
    node: &IndexAndWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.call_operator_loc());
    node.opening_loc());
    node.closing_loc());
    node.operator_loc());
    node.receiver());
    push_arguments_opt(&mut nodes, node.arguments());
    if let Some(block) = node.block() {
        block.as_node());
    }
    node.value());
    targets
}
pub fn comment_targets_of_index_operator_write_node<'sh>(
    node: &IndexOperatorWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.call_operator_loc());
    node.opening_loc());
    node.closing_loc());
    node.binary_operator_loc());
    node.receiver());
    push_arguments_opt(&mut nodes, node.arguments());
    if let Some(block) = node.block() {
        block.as_node());
    }
    node.value());
    targets
}
pub fn comment_targets_of_index_or_write_node<'sh>(
    node: &IndexOrWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.call_operator_loc());
    node.opening_loc());
    node.closing_loc());
    node.operator_loc());
    node.receiver());
    push_arguments_opt(&mut nodes, node.arguments());
    if let Some(block) = node.block() {
        block.as_node());
    }
    node.value());
    targets
}
pub fn comment_targets_of_index_target_node<'sh>(node: &IndexTargetNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.opening_loc());
    node.receiver());
    push_arguments_opt(&mut nodes, node.arguments());
    if let Some(block) = node.block() {
        block.as_node());
    }
    targets
}
pub fn comment_targets_of_instance_variable_and_write_node<'sh>(
    node: &InstanceVariableAndWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_instance_variable_operator_write_node<'sh>(
    node: &InstanceVariableOperatorWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.binary_operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_instance_variable_or_write_node<'sh>(
    node: &InstanceVariableOrWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_instance_variable_read_node<'sh>(
    node: &InstanceVariableReadNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_instance_variable_target_node<'sh>(
    node: &InstanceVariableTargetNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_instance_variable_write_node<'sh>(
    node: &InstanceVariableWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_integer_node<'sh>(node: &IntegerNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_interpolated_match_last_line_node<'sh>(
    node: &InterpolatedMatchLastLineNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.closing_loc());
    node.parts());
    targets
}
pub fn comment_targets_of_interpolated_regular_expression_node<'sh>(
    node: &InterpolatedRegularExpressionNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.closing_loc());
    node.parts());
    targets
}
pub fn comment_targets_of_interpolated_string_node<'sh>(
    node: &InterpolatedStringNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.closing_loc());
    node.parts());
    targets
}
pub fn comment_targets_of_interpolated_symbol_node<'sh>(
    node: &InterpolatedSymbolNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.closing_loc());
    node.parts());
    targets
}
pub fn comment_targets_of_interpolated_x_string_node<'sh>(
    node: &InterpolatedXStringNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.closing_loc());
    node.parts());
    targets
}
pub fn comment_targets_of_it_local_variable_read_node<'sh>(
    node: &ItLocalVariableReadNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_it_parameters_node<'sh>(
    node: &ItParametersNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_keyword_hash_node<'sh>(node: &KeywordHashNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.elements());
    targets
}
pub fn comment_targets_of_keyword_rest_parameter_node<'sh>(
    node: &KeywordRestParameterNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    targets
}
pub fn comment_targets_of_lambda_node<'sh>(node: &LambdaNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.opening_loc());
    node.closing_loc());
    node.parameters());
    node.body());
    targets
}
pub fn comment_targets_of_local_variable_and_write_node<'sh>(
    node: &LocalVariableAndWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_local_variable_operator_write_node<'sh>(
    node: &LocalVariableOperatorWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.binary_operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_local_variable_or_write_node<'sh>(
    node: &LocalVariableOrWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_local_variable_read_node<'sh>(
    node: &LocalVariableReadNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_local_variable_target_node<'sh>(
    node: &LocalVariableTargetNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_local_variable_write_node<'sh>(
    node: &LocalVariableWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_match_last_line_node<'sh>(
    node: &MatchLastLineNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.content_loc());
    node.closing_loc());
    targets
}
pub fn comment_targets_of_match_predicate_node<'sh>(
    node: &MatchPredicateNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.value());
    node.pattern());
    targets
}
pub fn comment_targets_of_match_required_node<'sh>(
    node: &MatchRequiredNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.value());
    node.pattern());
    targets
}
pub fn comment_targets_of_match_write_node<'sh>(node: &MatchWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.targets());
    targets
}
pub fn comment_targets_of_missing_node<'sh>(node: &MissingNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_module_node<'sh>(node: &ModuleNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.module_keyword_loc());
    node.end_keyword_loc());
    node.constant_path());
    node.body());
    targets
}
pub fn comment_targets_of_multi_target_node<'sh>(node: &MultiTargetNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.lparen_loc());
    node.rparen_loc());
    node.lefts());
    node.rest());
    node.rights());
    targets
}
pub fn comment_targets_of_multi_write_node<'sh>(node: &MultiWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.lparen_loc());
    node.rparen_loc());
    node.operator_loc());
    node.lefts());
    node.rest());
    node.rights());
    node.value());
    targets
}
pub fn comment_targets_of_next_node<'sh>(node: &NextNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.keyword_loc());
    push_arguments_opt(&mut nodes, node.arguments());
    targets
}
pub fn comment_targets_of_nil_node<'sh>(node: &NilNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_no_keywords_parameter_node<'sh>(
    node: &NoKeywordsParameterNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.keyword_loc());
    targets
}
pub fn comment_targets_of_numbered_parameters_node<'sh>(
    node: &NumberedParametersNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_numbered_reference_read_node<'sh>(
    node: &NumberedReferenceReadNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_optional_keyword_parameter_node<'sh>(
    node: &OptionalKeywordParameterNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.value());
    targets
}
pub fn comment_targets_of_optional_parameter_node<'sh>(
    node: &OptionalParameterNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    node.value());
    targets
}
pub fn comment_targets_of_or_node<'sh>(node: &OrNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.left());
    node.right());
    targets
}
pub fn comment_targets_of_parameters_node<'sh>(node: &ParametersNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.requireds());
    node.optionals());
    node.rest());
    node.posts());
    node.keywords());
    node.keyword_rest());
    if let Some(block) = node.block() {
        block.as_node());
    }
    targets
}
pub fn comment_targets_of_parentheses_node<'sh>(node: &ParenthesesNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.closing_loc());
    node.body());
    targets
}
pub fn comment_targets_of_pinned_expression_node<'sh>(
    node: &PinnedExpressionNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.lparen_loc());
    node.rparen_loc());
    node.expression());
    targets
}
pub fn comment_targets_of_pinned_variable_node<'sh>(
    node: &PinnedVariableNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.variable());
    targets
}
pub fn comment_targets_of_post_execution_node<'sh>(
    node: &PostExecutionNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.keyword_loc());
    node.opening_loc());
    node.closing_loc());
    push_statements_opt(&mut nodes, node.statements());
    targets
}
pub fn comment_targets_of_pre_execution_node<'sh>(
    node: &PreExecutionNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.keyword_loc());
    node.opening_loc());
    node.closing_loc());
    push_statements_opt(&mut nodes, node.statements());
    targets
}
pub fn comment_targets_of_program_node<'sh>(node: &ProgramNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.statements().as_node());
    targets
}
pub fn comment_targets_of_range_node<'sh>(node: &RangeNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.left());
    node.right());
    targets
}
pub fn comment_targets_of_rational_node<'sh>(node: &RationalNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_redo_node<'sh>(node: &RedoNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_regular_expression_node<'sh>(
    node: &RegularExpressionNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.content_loc());
    node.closing_loc());
    targets
}
pub fn comment_targets_of_required_keyword_parameter_node<'sh>(
    node: &RequiredKeywordParameterNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    targets
}
pub fn comment_targets_of_required_parameter_node<'sh>(
    node: &RequiredParameterNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_rescue_modifier_node<'sh>(
    node: &RescueModifierNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.keyword_loc());
    node.expression());
    node.rescue_expression());
    targets
}
pub fn comment_targets_of_rescue_node<'sh>(node: &RescueNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.keyword_loc());
    node.operator_loc());
    node.then_keyword_loc());
    node.exceptions());
    node.reference());
    push_statements_opt(&mut nodes, node.statements());
    push_rescue_clauses_opt(&mut nodes, node.subsequent());
    targets
}
pub fn comment_targets_of_rest_parameter_node<'sh>(
    node: &RestParameterNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.name_loc());
    node.operator_loc());
    targets
}
pub fn comment_targets_of_retry_node<'sh>(node: &RetryNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_return_node<'sh>(node: &ReturnNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.keyword_loc());
    push_arguments_opt(&mut nodes, node.arguments());
    targets
}
pub fn comment_targets_of_self_node<'sh>(node: &SelfNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_shareable_constant_node<'sh>(
    node: &ShareableConstantNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.write());
    targets
}
pub fn comment_targets_of_singleton_class_node<'sh>(
    node: &SingletonClassNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.class_keyword_loc());
    node.operator_loc());
    node.end_keyword_loc());
    node.expression());
    node.body());
    targets
}
pub fn comment_targets_of_source_encoding_node<'sh>(
    node: &SourceEncodingNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_source_file_node<'sh>(node: &SourceFileNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_source_line_node<'sh>(node: &SourceLineNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_splat_node<'sh>(node: &SplatNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.operator_loc());
    node.expression());
    targets
}
pub fn comment_targets_of_statements_node<'sh>(node: &StatementsNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.body());
    targets
}
pub fn comment_targets_of_string_node<'sh>(node: &StringNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.content_loc());
    node.closing_loc());
    targets
}
pub fn comment_targets_of_super_node<'sh>(node: &SuperNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.keyword_loc());
    node.lparen_loc());
    node.rparen_loc());
    push_arguments_opt(&mut nodes, node.arguments());
    node.block());
    targets
}
pub fn comment_targets_of_symbol_node<'sh>(node: &SymbolNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.value_loc());
    node.closing_loc());
    targets
}
pub fn comment_targets_of_true_node<'sh>(node: &TrueNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.location());
    targets
}
pub fn comment_targets_of_undef_node<'sh>(node: &UndefNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.keyword_loc());
    node.names());
    targets
}
pub fn comment_targets_of_unless_node<'sh>(node: &UnlessNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.keyword_loc());
    node.then_keyword_loc());
    node.end_keyword_loc());
    node.predicate());
    push_statements_opt(&mut nodes, node.statements());
    push_else_clause_opt(&mut nodes, node.else_clause());
    targets
}
pub fn comment_targets_of_until_node<'sh>(node: &UntilNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.keyword_loc());
    node.do_keyword_loc());
    node.closing_loc());
    node.predicate());
    push_statements_opt(&mut nodes, node.statements());
    targets
}
pub fn comment_targets_of_when_node<'sh>(node: &WhenNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.keyword_loc());
    node.then_keyword_loc());
    node.conditions());
    push_statements_opt(&mut nodes, node.statements());
    targets
}
pub fn comment_targets_of_while_node<'sh>(node: &WhileNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.keyword_loc());
    node.do_keyword_loc());
    node.closing_loc());
    node.predicate());
    push_statements_opt(&mut nodes, node.statements());
    targets
}
pub fn comment_targets_of_x_string_node<'sh>(node: &XStringNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.opening_loc());
    node.content_loc());
    node.closing_loc());
    targets
}
pub fn comment_targets_of_yield_node<'sh>(node: &YieldNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    node.keyword_loc());
    node.lparen_loc());
    node.rparen_loc());
    push_arguments_opt(&mut nodes, node.arguments());
    targets
}
*/
