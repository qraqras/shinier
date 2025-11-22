// https://github.com/ruby/prism/blob/main/lib/prism/parse_result/comments.rb
use ruby_prism::*;
use std::collections::HashMap;

/// Placement of comments relative to a target node or location.
/// - **leading**: comments before the target
/// - **trailing**: comments after the target
/// - **dangling**: comments inside the target
pub struct CommentPlacement<'sh> {
    leading: Option<Vec<CommentWrapper<'sh>>>,
    trailing: Option<Vec<CommentWrapper<'sh>>>,
    dangling: Option<Vec<CommentWrapper<'sh>>>,
}

/// Position of a comment relative to code.
/// - **OwnLine**: comment is on its own line
/// - **EndOfLine**: comment is at the end of a line of code
pub enum CommentPosition {
    OwnLine,
    EndOfLine,
}

/// Wrapper for a comment with its position relative to code.
pub struct CommentWrapper<'sh> {
    pub comment: Comment<'sh>,
    pub position: CommentPosition,
}
impl<'sh> From<(Comment<'sh>, CommentPosition)> for CommentWrapper<'sh> {
    fn from((comment, position): (Comment<'sh>, CommentPosition)) -> Self {
        Self { comment, position }
    }
}

/// Store for comments attached to targets.
pub struct CommentStore<'sh> {
    pub placement: HashMap<(usize, usize), CommentPlacement<'sh>>,
}
impl<'sh> CommentStore<'sh> {
    fn new() -> Self {
        Self {
            placement: HashMap::new(),
        }
    }
    /// Pushes a leading comment for a given target.
    fn push_leading(&mut self, target: &Target<'sh>, comment_wrapper: CommentWrapper<'sh>) {
        self.placement
            .entry((target.start_offset(), target.end_offset()))
            .or_insert_with(|| CommentPlacement {
                leading: Some(Vec::new()),
                trailing: None,
                dangling: None,
            })
            .leading
            .as_mut()
            .unwrap()
            .push(comment_wrapper);
    }
    /// Pushes a trailing comment for a given target.
    fn push_trailing(&mut self, target: &Target<'sh>, comment_wrapper: CommentWrapper<'sh>) {
        self.placement
            .entry((target.start_offset(), target.end_offset()))
            .or_insert_with(|| CommentPlacement {
                leading: None,
                trailing: Some(Vec::new()),
                dangling: None,
            })
            .trailing
            .as_mut()
            .unwrap()
            .push(comment_wrapper);
    }
    /// Pushes a dangling comment for a given target.
    fn push_dangling(&mut self, target: &Target<'sh>, comment_wrapper: CommentWrapper<'sh>) {
        self.placement
            .entry((target.start_offset(), target.end_offset()))
            .or_insert_with(|| CommentPlacement {
                leading: None,
                trailing: None,
                dangling: Some(Vec::new()),
            })
            .dangling
            .as_mut()
            .unwrap()
            .push(comment_wrapper);
    }
    /// Pops leading comments for a given target.
    pub fn pop_leadings(
        &mut self,
        target_start_offset: usize,
        target_end_offset: usize,
    ) -> Option<Vec<CommentWrapper<'sh>>> {
        self.placement
            .get_mut(&(target_start_offset, target_end_offset))
            .and_then(|placement| placement.leading.take())
    }
    /// Pops trailing comments for a given target.
    pub fn pop_trailings(
        &mut self,
        target_start_offset: usize,
        target_end_offset: usize,
    ) -> Option<Vec<CommentWrapper<'sh>>> {
        self.placement
            .get_mut(&(target_start_offset, target_end_offset))
            .and_then(|placement| placement.trailing.take())
    }
    /// Pops dangling comments for a given target.
    pub fn pop_danglings(
        &mut self,
        target_start_offset: usize,
        target_end_offset: usize,
    ) -> Option<Vec<CommentWrapper<'sh>>> {
        self.placement
            .get_mut(&(target_start_offset, target_end_offset))
            .and_then(|placement| placement.dangling.take())
    }
}

/// target node for attaching comments
pub enum Target<'sh> {
    ClosingLocation(Location<'sh>),
    Location(Location<'sh>),
    Node(Node<'sh>),
    OpeningLocation(Location<'sh>),
}
impl<'sh> Target<'sh> {
    // TODO: from_*を整理する
    pub fn from_node(node: Node<'sh>) -> Self {
        Self::Node(node)
    }
    pub fn from_loc(loc: Location<'sh>) -> Self {
        Self::Location(loc)
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
    fn is_enclosing(&self, start_offset: usize, end_offset: usize) -> bool {
        match self.is_node() {
            true => self.start_offset() <= start_offset && end_offset <= self.end_offset(),
            false => false,
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
impl<'sh> From<Location<'sh>> for Target<'sh> {
    fn from(loc: Location<'sh>) -> Self {
        Target::Location(loc)
    }
}
impl<'sh> From<Node<'sh>> for Target<'sh> {
    fn from(node: Node<'sh>) -> Self {
        Target::Node(node)
    }
}

/// attach comments to nodes and locations
pub fn attach<'sh>(parse_result: &'sh ParseResult<'sh>) -> CommentStore<'sh> {
    let mut comment_store = CommentStore::new();
    let source = parse_result.source();
    for comment in parse_result.comments() {
        let (preceding, enclosing, following) = nearest_targets(
            Target::Node(parse_result.node()),
            comment.location().start_offset(),
            comment.location().end_offset(),
        );
        match is_trailing(&comment, source) {
            true => match (preceding, enclosing, following) {
                (Some(p), _, _) => {
                    comment_store.push_trailing(&p, CommentWrapper::from((comment, CommentPosition::EndOfLine)));
                }
                (None, _, Some(f)) => {
                    comment_store.push_leading(&f, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                }
                (None, Some(e), None) => {
                    comment_store.push_leading(&e, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                }
                (None, None, None) => {
                    comment_store.push_leading(
                        &Target::from_node(parse_result.node()),
                        CommentWrapper::from((comment, CommentPosition::OwnLine)),
                    );
                }
            },
            false => match (preceding, enclosing, following) {
                (Some(p), _, Some(f)) => match (&p, &f) {
                    (Target::OpeningLocation(o), Target::ClosingLocation(c)) => {
                        comment_store.push_dangling(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                    }
                    (_, Target::ClosingLocation(c)) => {
                        comment_store.push_trailing(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                    }
                    (_, _) => {
                        comment_store.push_leading(&f, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                    }
                },
                (Some(p), _, None) => {
                    comment_store.push_dangling(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                }
                (None, _, Some(f)) => {
                    comment_store.push_leading(&f, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                }
                (None, Some(e), None) => {
                    comment_store.push_leading(&e, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                }
                (None, None, None) => {
                    comment_store.push_leading(
                        &Target::from_node(parse_result.node()),
                        CommentWrapper::from((comment, CommentPosition::OwnLine)),
                    );
                }
            },
        }
    }
    comment_store
}

/// find nearest targets for a comment
fn nearest_targets<'sh>(
    target: Target<'sh>,
    comment_start_offset: usize,
    comment_end_offset: usize,
) -> (Option<Target<'sh>>, Option<Target<'sh>>, Option<Target<'sh>>) {
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
            if current_target.is_enclosing(comment_start_offset, comment_end_offset) {
                let enclosing_target = targets.swap_remove(middle);
                return nearest_targets(enclosing_target, comment_start_offset, comment_end_offset);
            }
            // preceding
            if current_target_end <= comment_start_offset {
                preceding = Some(middle);
                left = middle + 1;
                continue;
            }
            // following
            if comment_end_offset <= current_target_start {
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

#[rustfmt::skip]
fn comment_targets<'sh>(target: &Target<'sh>) -> Vec<Target<'sh>> {
    match target {
        Target::ClosingLocation(_) => Vec::new(),
        Target::Location(_) => Vec::new(),
        Target::Node(node) => match node {
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
        Target::OpeningLocation(_) => Vec::new(),
    }
}

fn push_loc_closing<'sh>(loc: Option<Location<'sh>>, targets: &mut Vec<Target<'sh>>) {
    match loc {
        Some(loc) => {
            targets.push(Target::ClosingLocation(loc));
        }
        None => {}
    }
}
fn push_loc<'sh>(loc: Option<Location<'sh>>, targets: &mut Vec<Target<'sh>>) {
    match loc {
        Some(loc) => {
            targets.push(Target::Location(loc));
        }
        None => {}
    }
}
fn push_node<'sh>(node: Option<Node<'sh>>, targets: &mut Vec<Target<'sh>>) {
    match node {
        Some(node) => {
            targets.push(Target::Node(node));
        }
        None => {}
    }
}
fn push_loc_opening<'sh>(loc: Option<Location<'sh>>, targets: &mut Vec<Target<'sh>>) {
    match loc {
        Some(loc) => {
            targets.push(Target::OpeningLocation(loc));
        }
        None => {}
    }
}
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
    push_loc(Some(node.keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_alias_method_node<'sh>(node: &AliasMethodNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.new_name()), &mut targets);
    push_node(Some(node.old_name()), &mut targets);
    push_loc(Some(node.keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_alternation_pattern_node<'sh>(node: &AlternationPatternNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.left()), &mut targets);
    push_node(Some(node.right()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_and_node<'sh>(node: &AndNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.left()), &mut targets);
    push_node(Some(node.right()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
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
    push_loc(node.operator_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_assoc_splat_node<'sh>(node: &AssocSplatNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.value(), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_back_reference_read_node<'sh>(node: &BackReferenceReadNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_begin_node<'sh>(node: &BeginNode<'sh>) -> Vec<Target<'sh>> {
    fn rescue_clause_recursively<'sh>(rescue_node: &Option<RescueNode<'sh>>, targets: &mut Vec<Target<'sh>>) {
        match rescue_node {
            None => {}
            Some(rescue_node) => {
                push_loc_opening(Some(rescue_node.keyword_loc()), targets);
                push_nodelist(Some(rescue_node.exceptions()), targets);
                push_loc(rescue_node.operator_loc(), targets);
                push_node(rescue_node.reference(), targets);
                push_loc(rescue_node.then_keyword_loc(), targets);
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
                push_loc_closing(else_node.end_keyword_loc(), targets);
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
    push_loc_opening(node.begin_keyword_loc(), &mut targets);
    push_node(node.statements().map(|stmts| stmts.as_node()), &mut targets);
    rescue_clause_recursively(&node.rescue_clause(), &mut targets);
    else_clause(&node.else_clause(), &mut targets);
    ensure_clause(&node.ensure_clause(), &mut targets);
    push_loc_closing(node.end_keyword_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_block_argument_node<'sh>(node: &BlockArgumentNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(node.expression(), &mut targets);
    targets
}
pub fn comment_targets_of_block_local_variable_node<'sh>(node: &BlockLocalVariableNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_block_node<'sh>(node: &BlockNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.parameters(), &mut targets);
    push_node(node.body(), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_block_parameter_node<'sh>(node: &BlockParameterNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(node.name_loc(), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_block_parameters_node<'sh>(node: &BlockParametersNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.parameters().map(|node| node.as_node()), &mut targets);
    push_nodelist(Some(node.locals()), &mut targets);
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}

pub fn comment_targets_of_break_node<'sh>(node: &BreakNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.arguments().map(|node| node.as_node()), &mut targets);
    push_loc(Some(node.keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_call_and_write_node<'sh>(node: &CallAndWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.receiver(), &mut targets);
    push_loc(node.call_operator_loc(), &mut targets);
    push_loc(node.message_loc(), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_call_node<'sh>(node: &CallNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.receiver(), &mut targets);
    push_loc(node.call_operator_loc(), &mut targets);
    push_loc(node.message_loc(), &mut targets);
    push_loc_opening(node.opening_loc(), &mut targets);
    push_node(node.arguments().map(|node| node.as_node()), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    node.block().map(|node| targets.push(Target::Node(node)));
    targets
}
pub fn comment_targets_of_call_operator_write_node<'sh>(node: &CallOperatorWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.receiver(), &mut targets);
    push_loc(node.call_operator_loc(), &mut targets);
    push_loc(node.message_loc(), &mut targets);
    push_loc(Some(node.binary_operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_call_or_write_node<'sh>(node: &CallOrWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.receiver(), &mut targets);
    push_loc(node.call_operator_loc(), &mut targets);
    push_loc(node.message_loc(), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_call_target_node<'sh>(node: &CallTargetNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.receiver()), &mut targets);
    push_loc(Some(node.call_operator_loc()), &mut targets);
    push_loc(Some(node.message_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_capture_pattern_node<'sh>(node: &CapturePatternNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.value()), &mut targets);
    push_node(Some(node.target().as_node()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_case_match_node<'sh>(node: &CaseMatchNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.predicate(), &mut targets);
    push_nodelist(Some(node.conditions()), &mut targets);
    push_node(
        node.else_clause().map(|else_clause| else_clause.as_node()),
        &mut targets,
    );
    push_loc_opening(Some(node.case_keyword_loc()), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_case_node<'sh>(node: &CaseNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.predicate(), &mut targets);
    push_nodelist(Some(node.conditions()), &mut targets);
    push_node(
        node.else_clause().map(|else_clause| else_clause.as_node()),
        &mut targets,
    );
    push_loc_opening(Some(node.case_keyword_loc()), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_class_node<'sh>(node: &ClassNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.class_keyword_loc()), &mut targets);
    push_node(Some(node.constant_path()), &mut targets);
    push_loc(node.inheritance_operator_loc(), &mut targets);
    push_node(node.superclass(), &mut targets);
    push_node(node.body(), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_class_variable_and_write_node<'sh>(
    node: &ClassVariableAndWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_class_variable_operator_write_node<'sh>(
    node: &ClassVariableOperatorWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.binary_operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_class_variable_or_write_node<'sh>(node: &ClassVariableOrWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_class_variable_read_node<'sh>(node: &ClassVariableReadNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_class_variable_target_node<'sh>(node: &ClassVariableTargetNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_class_variable_write_node<'sh>(node: &ClassVariableWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_constant_and_write_node<'sh>(node: &ConstantAndWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_constant_operator_write_node<'sh>(node: &ConstantOperatorWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.binary_operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_constant_or_write_node<'sh>(node: &ConstantOrWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_constant_path_and_write_node<'sh>(node: &ConstantPathAndWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.target().as_node()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_constant_path_node<'sh>(node: &ConstantPathNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.parent(), &mut targets);
    push_loc(Some(node.delimiter_loc()), &mut targets);
    push_loc(Some(node.name_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_constant_path_operator_write_node<'sh>(
    node: &ConstantPathOperatorWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.target().as_node()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    push_loc(Some(node.binary_operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_constant_path_or_write_node<'sh>(node: &ConstantPathOrWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.target().as_node()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_constant_path_target_node<'sh>(node: &ConstantPathTargetNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.parent(), &mut targets);
    push_loc(Some(node.delimiter_loc()), &mut targets);
    push_loc(Some(node.name_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_constant_path_write_node<'sh>(node: &ConstantPathWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.target().as_node()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_constant_read_node<'sh>(node: &ConstantReadNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_constant_target_node<'sh>(node: &ConstantTargetNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_constant_write_node<'sh>(node: &ConstantWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_def_node<'sh>(node: &DefNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_node(node.receiver(), &mut targets);
    push_node(node.parameters().map(|p| p.as_node()), &mut targets);
    push_node(node.body(), &mut targets);
    push_loc_opening(Some(node.def_keyword_loc()), &mut targets);
    push_loc(node.operator_loc(), &mut targets);
    push_loc_opening(node.lparen_loc(), &mut targets);
    push_loc_closing(node.rparen_loc(), &mut targets);
    push_loc(node.equal_loc(), &mut targets);
    push_loc_closing(node.end_keyword_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_defined_node<'sh>(node: &DefinedNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(node.lparen_loc(), &mut targets);
    push_node(Some(node.value()), &mut targets);
    push_loc_closing(node.rparen_loc(), &mut targets);
    push_loc(Some(node.keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_else_node<'sh>(node: &ElseNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.else_keyword_loc()), &mut targets);
    push_node(node.statements().map(|s| s.as_node()), &mut targets);
    push_loc_closing(node.end_keyword_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_embedded_statements_node<'sh>(node: &EmbeddedStatementsNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_node(node.statements().map(|s| s.as_node()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_embedded_variable_node<'sh>(node: &EmbeddedVariableNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.variable()), &mut targets);
    targets
}
pub fn comment_targets_of_ensure_node<'sh>(node: &EnsureNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.ensure_keyword_loc()), &mut targets);
    push_node(node.statements().map(|s| s.as_node()), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_false_node<'sh>(node: &FalseNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_find_pattern_node<'sh>(node: &FindPatternNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.constant(), &mut targets);
    push_node(Some(node.left().as_node()), &mut targets);
    push_nodelist(Some(node.requireds()), &mut targets);
    push_node(Some(node.right()), &mut targets);
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_flip_flop_node<'sh>(node: &FlipFlopNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.left(), &mut targets);
    push_node(node.right(), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_float_node<'sh>(node: &FloatNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_for_node<'sh>(node: &ForNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.index()), &mut targets);
    push_node(Some(node.collection()), &mut targets);
    push_node(node.statements().map(|s| s.as_node()), &mut targets);
    push_loc_opening(Some(node.for_keyword_loc()), &mut targets);
    push_loc(Some(node.in_keyword_loc()), &mut targets);
    push_loc(node.do_keyword_loc(), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_forwarding_arguments_node<'sh>(node: &ForwardingArgumentsNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_forwarding_parameter_node<'sh>(node: &ForwardingParameterNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_forwarding_super_node<'sh>(node: &ForwardingSuperNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.block().map(|b| b.as_node()), &mut targets);
    targets
}
pub fn comment_targets_of_global_variable_and_write_node<'sh>(
    node: &GlobalVariableAndWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_global_variable_operator_write_node<'sh>(
    node: &GlobalVariableOperatorWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.binary_operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_global_variable_or_write_node<'sh>(
    node: &GlobalVariableOrWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_global_variable_read_node<'sh>(node: &GlobalVariableReadNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_global_variable_target_node<'sh>(node: &GlobalVariableTargetNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_global_variable_write_node<'sh>(node: &GlobalVariableWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_hash_node<'sh>(node: &HashNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    push_nodelist(Some(node.elements()), &mut targets);
    targets
}
pub fn comment_targets_of_hash_pattern_node<'sh>(node: &HashPatternNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.constant(), &mut targets);
    push_nodelist(Some(node.elements()), &mut targets);
    push_node(node.rest(), &mut targets);
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_if_node<'sh>(node: &IfNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(node.if_keyword_loc(), &mut targets);
    push_node(Some(node.predicate()), &mut targets);
    push_loc(node.then_keyword_loc(), &mut targets);
    push_node(node.statements().map(|s| s.as_node()), &mut targets);
    push_node(node.subsequent(), &mut targets);
    push_loc_closing(node.end_keyword_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_imaginary_node<'sh>(node: &ImaginaryNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.numeric()), &mut targets);
    targets
}
pub fn comment_targets_of_implicit_node<'sh>(node: &ImplicitNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_implicit_rest_node<'sh>(node: &ImplicitRestNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_in_node<'sh>(node: &InNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.pattern()), &mut targets);
    push_node(node.statements().map(|s| s.as_node()), &mut targets);
    push_loc_opening(Some(node.in_loc()), &mut targets);
    push_loc(node.then_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_index_and_write_node<'sh>(node: &IndexAndWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.receiver(), &mut targets);
    push_loc(node.call_operator_loc(), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_node(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    push_node(node.block().map(|b| b.as_node()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_index_operator_write_node<'sh>(node: &IndexOperatorWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.receiver(), &mut targets);
    push_loc(node.call_operator_loc(), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_node(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    push_node(node.block().map(|b| b.as_node()), &mut targets);
    push_loc(Some(node.binary_operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_index_or_write_node<'sh>(node: &IndexOrWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.receiver(), &mut targets);
    push_loc(node.call_operator_loc(), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_node(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    push_node(node.block().map(|b| b.as_node()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_index_target_node<'sh>(node: &IndexTargetNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.receiver()), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_node(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    push_node(node.block().map(|b| b.as_node()), &mut targets);
    targets
}
pub fn comment_targets_of_instance_variable_and_write_node<'sh>(
    node: &InstanceVariableAndWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_instance_variable_operator_write_node<'sh>(
    node: &InstanceVariableOperatorWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.binary_operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_instance_variable_or_write_node<'sh>(
    node: &InstanceVariableOrWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_instance_variable_read_node<'sh>(node: &InstanceVariableReadNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_instance_variable_target_node<'sh>(
    node: &InstanceVariableTargetNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_instance_variable_write_node<'sh>(node: &InstanceVariableWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_integer_node<'sh>(node: &IntegerNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_interpolated_match_last_line_node<'sh>(
    node: &InterpolatedMatchLastLineNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_nodelist(Some(node.parts()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_interpolated_regular_expression_node<'sh>(
    node: &InterpolatedRegularExpressionNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.opening_loc()), &mut targets);
    push_nodelist(Some(node.parts()), &mut targets);
    push_loc(Some(node.closing_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_interpolated_string_node<'sh>(node: &InterpolatedStringNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(node.opening_loc(), &mut targets);
    push_nodelist(Some(node.parts()), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_interpolated_symbol_node<'sh>(node: &InterpolatedSymbolNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(node.opening_loc(), &mut targets);
    push_nodelist(Some(node.parts()), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_interpolated_x_string_node<'sh>(node: &InterpolatedXStringNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_nodelist(Some(node.parts()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_it_local_variable_read_node<'sh>(node: &ItLocalVariableReadNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_it_parameters_node<'sh>(node: &ItParametersNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_keyword_hash_node<'sh>(node: &KeywordHashNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_nodelist(Some(node.elements()), &mut targets);
    targets
}
pub fn comment_targets_of_keyword_rest_parameter_node<'sh>(node: &KeywordRestParameterNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(node.name_loc(), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_lambda_node<'sh>(node: &LambdaNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.operator_loc()), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    push_node(node.parameters(), &mut targets);
    push_node(node.body(), &mut targets);
    targets
}
pub fn comment_targets_of_local_variable_and_write_node<'sh>(
    node: &LocalVariableAndWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_local_variable_operator_write_node<'sh>(
    node: &LocalVariableOperatorWriteNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.binary_operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_local_variable_or_write_node<'sh>(node: &LocalVariableOrWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_local_variable_read_node<'sh>(node: &LocalVariableReadNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_local_variable_target_node<'sh>(node: &LocalVariableTargetNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_local_variable_write_node<'sh>(node: &LocalVariableWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_match_last_line_node<'sh>(node: &MatchLastLineNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc(Some(node.content_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_match_predicate_node<'sh>(node: &MatchPredicateNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.value()), &mut targets);
    push_node(Some(node.pattern()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_match_required_node<'sh>(node: &MatchRequiredNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.value()), &mut targets);
    push_node(Some(node.pattern()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_match_write_node<'sh>(node: &MatchWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.call().as_node()), &mut targets);
    push_nodelist(Some(node.targets()), &mut targets);
    targets
}
pub fn comment_targets_of_missing_node<'sh>(node: &MissingNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_module_node<'sh>(node: &ModuleNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.module_keyword_loc()), &mut targets);
    push_node(Some(node.constant_path()), &mut targets);
    push_node(node.body(), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_multi_target_node<'sh>(node: &MultiTargetNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_nodelist(Some(node.lefts()), &mut targets);
    push_node(node.rest(), &mut targets);
    push_nodelist(Some(node.rights()), &mut targets);
    push_loc_opening(node.lparen_loc(), &mut targets);
    push_loc_closing(node.rparen_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_multi_write_node<'sh>(node: &MultiWriteNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_nodelist(Some(node.lefts()), &mut targets);
    push_node(node.rest(), &mut targets);
    push_nodelist(Some(node.rights()), &mut targets);
    push_loc_opening(node.lparen_loc(), &mut targets);
    push_loc_closing(node.rparen_loc(), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_next_node<'sh>(node: &NextNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc(Some(node.keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_nil_node<'sh>(node: &NilNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_no_keywords_parameter_node<'sh>(node: &NoKeywordsParameterNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.operator_loc()), &mut targets);
    push_loc(Some(node.keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_numbered_parameters_node<'sh>(node: &NumberedParametersNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_numbered_reference_read_node<'sh>(node: &NumberedReferenceReadNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_optional_keyword_parameter_node<'sh>(
    node: &OptionalKeywordParameterNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_optional_parameter_node<'sh>(node: &OptionalParameterNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.value()), &mut targets);
    targets
}
pub fn comment_targets_of_or_node<'sh>(node: &OrNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.left()), &mut targets);
    push_node(Some(node.right()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_parameters_node<'sh>(node: &ParametersNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_nodelist(Some(node.requireds()), &mut targets);
    push_nodelist(Some(node.optionals()), &mut targets);
    push_node(node.rest(), &mut targets);
    push_nodelist(Some(node.posts()), &mut targets);
    push_nodelist(Some(node.keywords()), &mut targets);
    push_node(node.keyword_rest(), &mut targets);
    push_node(node.block().map(|b| b.as_node()), &mut targets);
    targets
}
pub fn comment_targets_of_parentheses_node<'sh>(node: &ParenthesesNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.body(), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_pinned_expression_node<'sh>(node: &PinnedExpressionNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.expression()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_loc_opening(Some(node.lparen_loc()), &mut targets);
    push_loc_closing(Some(node.rparen_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_pinned_variable_node<'sh>(node: &PinnedVariableNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.variable()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_post_execution_node<'sh>(node: &PostExecutionNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.statements().map(|s| s.as_node()), &mut targets);
    push_loc(Some(node.keyword_loc()), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_pre_execution_node<'sh>(node: &PreExecutionNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.statements().map(|s| s.as_node()), &mut targets);
    push_loc(Some(node.keyword_loc()), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_program_node<'sh>(node: &ProgramNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.statements().as_node()), &mut targets);
    targets
}
pub fn comment_targets_of_range_node<'sh>(node: &RangeNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(node.left(), &mut targets);
    push_node(node.right(), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_rational_node<'sh>(node: &RationalNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_redo_node<'sh>(node: &RedoNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_regular_expression_node<'sh>(node: &RegularExpressionNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc(Some(node.content_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_required_keyword_parameter_node<'sh>(
    node: &RequiredKeywordParameterNode<'sh>,
) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.name_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_required_parameter_node<'sh>(node: &RequiredParameterNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_rescue_modifier_node<'sh>(node: &RescueModifierNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.expression()), &mut targets);
    push_loc(Some(node.keyword_loc()), &mut targets);
    push_node(Some(node.rescue_expression()), &mut targets);
    targets
}
pub fn comment_targets_of_rescue_node<'sh>(node: &RescueNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.keyword_loc()), &mut targets);
    push_nodelist(Some(node.exceptions()), &mut targets);
    push_loc(node.operator_loc(), &mut targets);
    push_node(node.reference(), &mut targets);
    push_loc(node.then_keyword_loc(), &mut targets);
    push_node(node.statements().map(|s| s.as_node()), &mut targets);
    push_node(node.subsequent().map(|s| s.as_node()), &mut targets);
    targets
}
pub fn comment_targets_of_rest_parameter_node<'sh>(node: &RestParameterNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(node.name_loc(), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_retry_node<'sh>(node: &RetryNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_return_node<'sh>(node: &ReturnNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.keyword_loc()), &mut targets);
    push_node(node.arguments().map(|a| a.as_node()), &mut targets);
    targets
}
pub fn comment_targets_of_self_node<'sh>(node: &SelfNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_shareable_constant_node<'sh>(node: &ShareableConstantNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_node(Some(node.write()), &mut targets);
    targets
}
pub fn comment_targets_of_singleton_class_node<'sh>(node: &SingletonClassNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.class_keyword_loc()), &mut targets);
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(Some(node.expression()), &mut targets);
    push_node(node.body(), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_source_encoding_node<'sh>(node: &SourceEncodingNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_source_file_node<'sh>(node: &SourceFileNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_source_line_node<'sh>(node: &SourceLineNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_splat_node<'sh>(node: &SplatNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.operator_loc()), &mut targets);
    push_node(node.expression(), &mut targets);
    targets
}
pub fn comment_targets_of_statements_node<'sh>(node: &StatementsNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_nodelist(Some(node.body()), &mut targets);
    targets
}
pub fn comment_targets_of_string_node<'sh>(node: &StringNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc(Some(node.content_loc()), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_super_node<'sh>(node: &SuperNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.keyword_loc()), &mut targets);
    push_loc_opening(node.lparen_loc(), &mut targets);
    push_node(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc_closing(node.rparen_loc(), &mut targets);
    push_node(node.block(), &mut targets);
    targets
}
pub fn comment_targets_of_symbol_node<'sh>(node: &SymbolNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc(node.value_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_true_node<'sh>(node: &TrueNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc(Some(node.location()), &mut targets);
    targets
}
pub fn comment_targets_of_undef_node<'sh>(node: &UndefNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_nodelist(Some(node.names()), &mut targets);
    push_loc(Some(node.keyword_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_unless_node<'sh>(node: &UnlessNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.keyword_loc()), &mut targets);
    push_node(Some(node.predicate()), &mut targets);
    push_loc(node.then_keyword_loc(), &mut targets);
    push_node(node.statements().map(|s| s.as_node()), &mut targets);
    push_node(node.else_clause().map(|e| e.as_node()), &mut targets);
    push_loc_closing(node.end_keyword_loc(), &mut targets);
    targets
}
pub fn comment_targets_of_until_node<'sh>(node: &UntilNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.keyword_loc()), &mut targets);
    push_loc(node.do_keyword_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    push_node(Some(node.predicate()), &mut targets);
    push_node(node.statements().map(|s| s.as_node()), &mut targets);
    targets
}
pub fn comment_targets_of_when_node<'sh>(node: &WhenNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.keyword_loc()), &mut targets);
    push_nodelist(Some(node.conditions()), &mut targets);
    push_loc(node.then_keyword_loc(), &mut targets);
    push_node(node.statements().map(|s| s.as_node()), &mut targets);
    targets
}
pub fn comment_targets_of_while_node<'sh>(node: &WhileNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.keyword_loc()), &mut targets);
    push_loc(node.do_keyword_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    push_node(Some(node.predicate()), &mut targets);
    push_node(node.statements().map(|s| s.as_node()), &mut targets);
    targets
}
pub fn comment_targets_of_x_string_node<'sh>(node: &XStringNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc(Some(node.content_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
pub fn comment_targets_of_yield_node<'sh>(node: &YieldNode<'sh>) -> Vec<Target<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.keyword_loc()), &mut targets);
    push_loc_opening(node.lparen_loc(), &mut targets);
    push_node(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc_closing(node.rparen_loc(), &mut targets);
    targets
}
