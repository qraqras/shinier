use ruby_prism::Visit;
use ruby_prism::*;
use std::collections::HashMap;

/// Placement of comments relative to a target node or location.
/// - **leading**: comments before the target
/// - **trailing**: comments after the target
/// - **dangling**: comments inside the target but not attached to any child nodes
///
/// Example:
/// ```ruby
/// # leading comment
/// def foo(param) # trailing comment
///   # dangling comment
/// end
/// ```
pub struct CommentPlacement<'sh> {
    leading: Option<Vec<CommentWrapper<'sh>>>,
    trailing: Option<Vec<CommentWrapper<'sh>>>,
    dangling: Option<Vec<CommentWrapper<'sh>>>,
}

/// Position of a comment relative to code.
/// - **OwnLine**: comment is on its own line
/// - **EndOfLine**: comment is at the end of a line of code
///
/// Example:
/// ```ruby
/// # OwnLine
/// def foo(param)
///   bar # EndOfLine
/// end
/// ```
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
    placements: HashMap<(usize, usize), CommentPlacement<'sh>>,
    sorted_locations: Vec<(usize, usize)>,
}
impl<'sh> CommentStore<'sh> {
    /// Creates a new CommentStore.
    fn new() -> Self {
        Self {
            placements: HashMap::new(),
            sorted_locations: Vec::new(),
        }
    }
    /// Pushes a leading comment for a given target.
    fn push_leading(&mut self, target: &CommentTarget<'sh>, comment_wrapper: CommentWrapper<'sh>) {
        self.placements
            .entry((target.start_offset(), target.end_offset()))
            .or_insert_with(|| CommentPlacement {
                leading: Some(Vec::new()),
                trailing: None,
                dangling: None,
            })
            .leading
            .get_or_insert_with(Vec::new)
            .push(comment_wrapper);
    }
    /// Pushes a trailing comment for a given target.
    fn push_trailing(&mut self, target: &CommentTarget<'sh>, comment_wrapper: CommentWrapper<'sh>) {
        self.placements
            .entry((target.start_offset(), target.end_offset()))
            .or_insert_with(|| CommentPlacement {
                leading: None,
                trailing: Some(Vec::new()),
                dangling: None,
            })
            .trailing
            .get_or_insert_with(Vec::new)
            .push(comment_wrapper);
    }
    /// Pushes a dangling comment for a given target.
    fn push_dangling(&mut self, target: &CommentTarget<'sh>, comment_wrapper: CommentWrapper<'sh>) {
        self.placements
            .entry((target.start_offset(), target.end_offset()))
            .or_insert_with(|| CommentPlacement {
                leading: None,
                trailing: None,
                dangling: Some(Vec::new()),
            })
            .dangling
            .get_or_insert_with(Vec::new)
            .push(comment_wrapper);
    }
    /// Takes leading comments for a given target.
    pub fn take_leadings(
        &mut self,
        target_start_offset: usize,
        target_end_offset: usize,
    ) -> Option<Vec<CommentWrapper<'sh>>> {
        self.placements
            .get_mut(&(target_start_offset, target_end_offset))
            .and_then(|placement| placement.leading.take())
    }
    /// Takes trailing comments for a given target.
    pub fn take_trailings(
        &mut self,
        target_start_offset: usize,
        target_end_offset: usize,
    ) -> Option<Vec<CommentWrapper<'sh>>> {
        self.placements
            .get_mut(&(target_start_offset, target_end_offset))
            .and_then(|placement| placement.trailing.take())
    }
    /// Takes dangling comments for a given target.
    pub fn take_danglings(
        &mut self,
        target_start_offset: usize,
        target_end_offset: usize,
    ) -> Option<Vec<CommentWrapper<'sh>>> {
        self.placements
            .get_mut(&(target_start_offset, target_end_offset))
            .and_then(|placement| placement.dangling.take())
    }
    /// Checks if there are leading comments for a given target.
    #[allow(dead_code)]
    pub fn has_leadings(&self, target_start_offset: usize, target_end_offset: usize) -> bool {
        self.placements
            .get(&(target_start_offset, target_end_offset))
            .and_then(|placement| placement.leading.as_ref())
            .is_some()
    }
    /// Checks if there are trailing comments for a given target.
    #[allow(dead_code)]
    pub fn has_trailings(&self, target_start_offset: usize, target_end_offset: usize) -> bool {
        self.placements
            .get(&(target_start_offset, target_end_offset))
            .and_then(|placement| placement.trailing.as_ref())
            .is_some()
    }
    /// Checks if there are dangling comments for a given target.
    #[allow(dead_code)]
    pub fn has_danglings(&self, target_start_offset: usize, target_end_offset: usize) -> bool {
        self.placements
            .get(&(target_start_offset, target_end_offset))
            .and_then(|placement| placement.dangling.as_ref())
            .is_some()
    }
    pub fn has_comments_between(&self, target_start_offset: usize, target_end_offset: usize) -> bool {
        let start_idx = self
            .sorted_locations
            .partition_point(|&(start, _)| start < target_start_offset);
        self.sorted_locations[start_idx..]
            .iter()
            .take_while(|&&(start, _)| start <= target_end_offset)
            .any(|&(start, end)| start >= target_start_offset && end <= target_end_offset)
    }
}

/// Type of target for comment attachment.
///
/// These types determine how comments are attached relative to code structures:
///
/// - **Opening**: Start of a block structure (e.g., `if`, `def`, `begin`, `(`, `{`)
/// - **OpeningLike**: Follows an opening, but may not start its own line
///   (e.g., method name after `def`, condition after `if`)
/// - **OpeningAndClosing**: Acts as closing for previous block and opening for next
///   (e.g., `else`, `elsif`, `rescue`, `ensure`, `when`, `in`)
/// - **OpeningLikeAndClosing**: Acts as closing but followed by content on same logical unit
///   (e.g., `)` in `def foo(param)` - closes params but part of def header)
/// - **Closing**: End of a block structure (e.g., `end`, `)`, `}`)
/// - **Regular**: Normal code elements without special block semantics
pub enum TargetType {
    Opening,
    OpeningLike,
    OpeningAndClosing,
    OpeningLikeAndClosing,
    Closing,
    Regular,
}

/// Target node or location for attaching comments.
pub enum CommentTarget<'sh> {
    Location(Location<'sh>, TargetType),
    Node(Node<'sh>, TargetType),
}
impl<'sh> CommentTarget<'sh> {
    /// Returns the start offset of the target.
    pub fn start_offset(&self) -> usize {
        match self {
            CommentTarget::Location(loc, _) => loc.start_offset(),
            CommentTarget::Node(node, _) => node.location().start_offset(),
        }
    }
    /// Returns the end offset of the target.
    pub fn end_offset(&self) -> usize {
        match self {
            CommentTarget::Location(loc, _) => loc.end_offset(),
            CommentTarget::Node(node, _) => node.location().end_offset(),
        }
    }
    /// Checks if the target encloses the given offsets.
    /// If the target is a location, it cannot enclose anything.
    fn is_enclosing(&self, start_offset: usize, end_offset: usize) -> bool {
        match self {
            CommentTarget::Location(_, _) => false,
            CommentTarget::Node(_, _) => self.start_offset() <= start_offset && end_offset <= self.end_offset(),
        }
    }
    /// Checks if the target has opening characteristic.
    #[allow(dead_code)]
    fn has_opening_characteristic(&self) -> bool {
        match self {
            CommentTarget::Location(_, TargetType::Opening) => true,
            CommentTarget::Location(_, TargetType::OpeningLike) => true,
            CommentTarget::Location(_, TargetType::OpeningAndClosing) => true,
            CommentTarget::Location(_, TargetType::OpeningLikeAndClosing) => true,
            CommentTarget::Node(_, TargetType::Opening) => true,
            CommentTarget::Node(_, TargetType::OpeningLike) => true,
            CommentTarget::Node(_, TargetType::OpeningAndClosing) => true,
            CommentTarget::Node(_, TargetType::OpeningLikeAndClosing) => true,
            _ => false,
        }
    }
    /// Checks if the target has closing characteristic.
    #[allow(dead_code)]
    fn has_closing_characteristic(&self) -> bool {
        match self {
            CommentTarget::Location(_, TargetType::OpeningAndClosing) => true,
            CommentTarget::Location(_, TargetType::OpeningLikeAndClosing) => true,
            CommentTarget::Location(_, TargetType::Closing) => true,
            CommentTarget::Node(_, TargetType::OpeningAndClosing) => true,
            CommentTarget::Node(_, TargetType::OpeningLikeAndClosing) => true,
            CommentTarget::Node(_, TargetType::Closing) => true,
            _ => false,
        }
    }
    /// Checks if the target is `Opening`.
    #[allow(dead_code)]
    fn is_opening(&self) -> bool {
        match self {
            CommentTarget::Location(_, TargetType::Opening) => true,
            CommentTarget::Location(_, TargetType::OpeningAndClosing) => true,
            CommentTarget::Node(_, TargetType::Opening) => true,
            CommentTarget::Node(_, TargetType::OpeningAndClosing) => true,
            _ => false,
        }
    }
    /// Checks if the target is `OpeningLike`.
    #[allow(dead_code)]
    fn is_opening_like(&self) -> bool {
        match self {
            CommentTarget::Location(_, TargetType::OpeningLike) => true,
            CommentTarget::Location(_, TargetType::OpeningLikeAndClosing) => true,
            CommentTarget::Node(_, TargetType::OpeningLike) => true,
            CommentTarget::Node(_, TargetType::OpeningLikeAndClosing) => true,
            _ => false,
        }
    }
    /// Checks if the target is `OpeningAndClosing`.
    #[allow(dead_code)]
    fn is_opening_and_closing(&self) -> bool {
        match self {
            CommentTarget::Location(_, TargetType::OpeningAndClosing) => true,
            CommentTarget::Node(_, TargetType::OpeningAndClosing) => true,
            _ => false,
        }
    }
    /// Checks if the target is `OpeningLikeAndClosing`.
    #[allow(dead_code)]
    fn is_opening_like_and_closing(&self) -> bool {
        match self {
            CommentTarget::Location(_, TargetType::OpeningLikeAndClosing) => true,
            CommentTarget::Node(_, TargetType::OpeningLikeAndClosing) => true,
            _ => false,
        }
    }
    /// Checks if the target is `Closing`.
    #[allow(dead_code)]
    fn is_closing(&self) -> bool {
        match self {
            CommentTarget::Location(_, TargetType::Closing) => true,
            CommentTarget::Location(_, TargetType::OpeningAndClosing) => true,
            CommentTarget::Location(_, TargetType::OpeningLikeAndClosing) => true,
            CommentTarget::Node(_, TargetType::Closing) => true,
            CommentTarget::Node(_, TargetType::OpeningAndClosing) => true,
            CommentTarget::Node(_, TargetType::OpeningLikeAndClosing) => true,
            _ => false,
        }
    }
    /// Checks if the target is `Regular`.
    #[allow(dead_code)]
    fn is_regular(&self) -> bool {
        match self {
            CommentTarget::Location(_, TargetType::Regular) => true,
            CommentTarget::Node(_, TargetType::Regular) => true,
            _ => false,
        }
    }
}
impl<'sh> PartialEq for CommentTarget<'sh> {
    fn eq(&self, other: &Self) -> bool {
        self.start_offset() == other.start_offset() && self.end_offset() == other.end_offset()
    }
}
impl<'sh> From<(Location<'sh>, TargetType)> for CommentTarget<'sh> {
    fn from((location, r#type): (Location<'sh>, TargetType)) -> Self {
        CommentTarget::Location(location, r#type)
    }
}
impl<'sh> From<(Node<'sh>, TargetType)> for CommentTarget<'sh> {
    fn from((node, r#type): (Node<'sh>, TargetType)) -> Self {
        CommentTarget::Node(node, r#type)
    }
}

/// Attach comments to nodes and locations.
pub fn attach<'sh>(parse_result: &'sh ParseResult<'sh>) -> CommentStore<'sh> {
    // Determines if a comment is a trailing comment.
    fn is_trailing_comment(comment: &Comment<'_>, source: &[u8]) -> bool {
        let mut curr = comment.location().start_offset();
        while curr > 0 {
            let next = curr - 1;
            if source[next] == b' ' || source[next] == b'\t' {
                curr = next;
                continue;
            }
            if source[next] == b'\n' {
                return false;
            }
            return true;
        }
        false
    }
    // Returns the column number (0-based) of the given start_offset.
    fn col(start_offset: usize, source: &[u8]) -> usize {
        let mut col = 0;
        let mut curr = start_offset;
        while curr > 0 {
            let next = curr - 1;
            if source[next] == b'\n' {
                break;
            }
            col += 1;
            curr = next;
        }
        col
    }
    let mut comment_store = CommentStore::new();
    for comment in parse_result.comments() {
        let comment_start_offset = comment.location().start_offset();
        let comment_end_offset = comment.location().end_offset();
        let (preceding, enclosing, following) = nearest_targets(
            CommentTarget::from((parse_result.node(), TargetType::Regular)),
            comment_start_offset,
            comment_end_offset,
        );
        match is_trailing_comment(&comment, parse_result.source()) {
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
                        &CommentTarget::from((parse_result.node(), TargetType::Regular)),
                        CommentWrapper::from((comment, CommentPosition::OwnLine)),
                    );
                }
            },
            false => match (preceding, enclosing, following) {
                (Some(p), Some(e), Some(f)) => {
                    // 開/閉の間にあるコメント
                    if p.has_opening_characteristic() && f.has_closing_characteristic() {
                        if f.is_opening_and_closing() {
                            let preceding_col = col(p.start_offset(), parse_result.source());
                            let following_col = col(f.start_offset(), parse_result.source());
                            let comment_col = col(comment.location().start_offset(), parse_result.source());
                            if preceding_col.min(following_col) < comment_col
                                && !comment_store.has_leadings(f.start_offset(), f.end_offset())
                            {
                                // 内側
                                if p.is_opening() && matches!(&p, CommentTarget::Node(_, _)) {
                                    comment_store
                                        .push_dangling(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                                } else {
                                    comment_store
                                        .push_dangling(&e, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                                }
                            } else {
                                // 外側
                                comment_store
                                    .push_leading(&f, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                            }
                        } else {
                            if p.is_opening() && matches!(&p, CommentTarget::Node(_, _)) {
                                comment_store
                                    .push_dangling(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                            } else {
                                comment_store
                                    .push_dangling(&e, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                            }
                        }
                    // 閉の前にあるコメント
                    } else if f.has_closing_characteristic() {
                        if f.is_opening_and_closing() {
                            let preceding_col = col(p.start_offset(), parse_result.source());
                            let following_col = col(f.start_offset(), parse_result.source());
                            let comment_col = col(comment.location().start_offset(), parse_result.source());
                            if preceding_col.min(following_col) < comment_col
                                && !comment_store.has_leadings(f.start_offset(), f.end_offset())
                            {
                                // 内側
                                comment_store
                                    .push_trailing(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                            } else {
                                // 外側
                                comment_store
                                    .push_leading(&f, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                            }
                        } else {
                            comment_store.push_trailing(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                        }
                    } else {
                        comment_store.push_leading(&f, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                    }
                }
                (Some(p), None, Some(f)) => {
                    // 開/閉の間にあるコメント
                    if p.has_opening_characteristic() && f.has_closing_characteristic() {
                        if f.is_opening_and_closing() {
                            let preceding_col = col(p.start_offset(), parse_result.source());
                            let following_col = col(f.start_offset(), parse_result.source());
                            let comment_col = col(comment.location().start_offset(), parse_result.source());
                            if preceding_col.min(following_col) < comment_col
                                && !comment_store.has_leadings(f.start_offset(), f.end_offset())
                            {
                                // 内側
                                if p.is_opening() && matches!(&p, CommentTarget::Node(_, _)) {
                                    comment_store
                                        .push_dangling(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                                } else {
                                    comment_store
                                        .push_trailing(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                                }
                            } else {
                                // 外側
                                comment_store
                                    .push_leading(&f, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                            }
                        } else {
                            if p.is_opening() && matches!(&p, CommentTarget::Node(_, _)) {
                                comment_store
                                    .push_dangling(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                            } else {
                                comment_store
                                    .push_trailing(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                            }
                        }
                    // 閉の前にあるコメント
                    } else if f.has_closing_characteristic() {
                        if f.is_opening_and_closing() {
                            let preceding_col = col(p.start_offset(), parse_result.source());
                            let following_col = col(f.start_offset(), parse_result.source());
                            let comment_col = col(comment.location().start_offset(), parse_result.source());
                            if preceding_col.min(following_col) < comment_col
                                && !comment_store.has_leadings(f.start_offset(), f.end_offset())
                            {
                                // 内側
                                comment_store
                                    .push_trailing(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                            } else {
                                // 外側
                                comment_store
                                    .push_leading(&f, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                            }
                        } else {
                            comment_store.push_trailing(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                        }
                    } else {
                        comment_store.push_leading(&f, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                    }
                }
                (Some(p), _, None) => {
                    comment_store.push_trailing(&p, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                }
                (None, _, Some(f)) => {
                    comment_store.push_leading(&f, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                }
                (None, Some(e), None) => {
                    comment_store.push_leading(&e, CommentWrapper::from((comment, CommentPosition::OwnLine)));
                }
                (None, None, None) => {
                    comment_store.push_leading(
                        &CommentTarget::from((parse_result.node(), TargetType::Regular)),
                        CommentWrapper::from((comment, CommentPosition::OwnLine)),
                    );
                }
            },
        }
        comment_store
            .sorted_locations
            .push((comment_start_offset, comment_end_offset));
    }
    comment_store.sorted_locations.sort_unstable();
    comment_store
}

/// Finds the nearest preceding, enclosing, and following targets for a comment.
fn nearest_targets<'sh>(
    target: CommentTarget<'sh>,
    comment_start_offset: usize,
    comment_end_offset: usize,
) -> (
    Option<CommentTarget<'sh>>,
    Option<CommentTarget<'sh>>,
    Option<CommentTarget<'sh>>,
) {
    {
        // Collect candidates
        let mut targets = collect_child_targets(&target);
        targets.sort_by_key(|t| t.start_offset());

        let mut preceding_index: Option<usize> = None;
        let mut following_index: Option<usize> = None;

        // Binary search for the nearest targets
        let mut left = 0;
        let mut right = targets.len();
        while left < right {
            let middle = (left + right) / 2;
            let current_target = &targets[middle];
            let current_target_start = current_target.start_offset();
            let current_target_end = current_target.end_offset();
            // Determine the enclosing target
            if current_target.is_enclosing(comment_start_offset, comment_end_offset) {
                let enclosing_target = targets.swap_remove(middle);
                return nearest_targets(enclosing_target, comment_start_offset, comment_end_offset);
            }
            // Determine the preceding target
            if current_target_end <= comment_start_offset {
                preceding_index = Some(middle);
                left = middle + 1;
                continue;
            }
            // Determine the following target
            if comment_end_offset <= current_target_start {
                following_index = Some(middle);
                right = middle;
                continue;
            }
            unreachable!("comment location overlaps with a target location");
        }
        let following = following_index.map(|idx| targets.remove(idx));
        let preceding = preceding_index.map(|idx| targets.remove(idx));
        (preceding, Some(target), following)
    }
}

/// Collects child targets of a given target.
#[rustfmt::skip]
fn collect_child_targets<'sh>(target: &CommentTarget<'sh>) -> Vec<CommentTarget<'sh>> {
    match target {
        CommentTarget::Location(_, _) => Vec::new(),
        CommentTarget::Node(node, _) => match node {
            Node::AliasGlobalVariableNode           { .. } => collect_child_targets_of_alias_global_variable_node           (&node.as_alias_global_variable_node().unwrap()           ),
            Node::AliasMethodNode                   { .. } => collect_child_targets_of_alias_method_node                    (&node.as_alias_method_node().unwrap()                    ),
            Node::AlternationPatternNode            { .. } => collect_child_targets_of_alternation_pattern_node             (&node.as_alternation_pattern_node().unwrap()             ),
            Node::AndNode                           { .. } => collect_child_targets_of_and_node                             (&node.as_and_node().unwrap()                             ),
            Node::ArgumentsNode                     { .. } => collect_child_targets_of_arguments_node                       (&node.as_arguments_node().unwrap()                       ),
            Node::ArrayNode                         { .. } => collect_child_targets_of_array_node                           (&node.as_array_node().unwrap()                           ),
            Node::ArrayPatternNode                  { .. } => collect_child_targets_of_array_pattern_node                   (&node.as_array_pattern_node().unwrap()                   ),
            Node::AssocNode                         { .. } => collect_child_targets_of_assoc_node                           (&node.as_assoc_node().unwrap()                           ),
            Node::AssocSplatNode                    { .. } => collect_child_targets_of_assoc_splat_node                     (&node.as_assoc_splat_node().unwrap()                     ),
            Node::BackReferenceReadNode             { .. } => collect_child_targets_of_back_reference_read_node             (&node.as_back_reference_read_node().unwrap()             ),
            Node::BeginNode                         { .. } => collect_child_targets_of_begin_node                           (&node.as_begin_node().unwrap()                           ),
            Node::BlockArgumentNode                 { .. } => collect_child_targets_of_block_argument_node                  (&node.as_block_argument_node().unwrap()                  ),
            Node::BlockLocalVariableNode            { .. } => collect_child_targets_of_block_local_variable_node            (&node.as_block_local_variable_node().unwrap()            ),
            Node::BlockNode                         { .. } => collect_child_targets_of_block_node                           (&node.as_block_node().unwrap()                           ),
            Node::BlockParameterNode                { .. } => collect_child_targets_of_block_parameter_node                 (&node.as_block_parameter_node().unwrap()                 ),
            Node::BlockParametersNode               { .. } => collect_child_targets_of_block_parameters_node                (&node.as_block_parameters_node().unwrap()                ),
            Node::BreakNode                         { .. } => collect_child_targets_of_break_node                           (&node.as_break_node().unwrap()                           ),
            Node::CallAndWriteNode                  { .. } => collect_child_targets_of_call_and_write_node                  (&node.as_call_and_write_node().unwrap()                  ),
            Node::CallNode                          { .. } => collect_child_targets_of_call_node                            (&node.as_call_node().unwrap()                            ),
            Node::CallOperatorWriteNode             { .. } => collect_child_targets_of_call_operator_write_node             (&node.as_call_operator_write_node().unwrap()             ),
            Node::CallOrWriteNode                   { .. } => collect_child_targets_of_call_or_write_node                   (&node.as_call_or_write_node().unwrap()                   ),
            Node::CallTargetNode                    { .. } => collect_child_targets_of_call_target_node                     (&node.as_call_target_node().unwrap()                     ),
            Node::CapturePatternNode                { .. } => collect_child_targets_of_capture_pattern_node                 (&node.as_capture_pattern_node().unwrap()                 ),
            Node::CaseMatchNode                     { .. } => collect_child_targets_of_case_match_node                      (&node.as_case_match_node().unwrap()                      ),
            Node::CaseNode                          { .. } => collect_child_targets_of_case_node                            (&node.as_case_node().unwrap()                            ),
            Node::ClassNode                         { .. } => collect_child_targets_of_class_node                           (&node.as_class_node().unwrap()                           ),
            Node::ClassVariableAndWriteNode         { .. } => collect_child_targets_of_class_variable_and_write_node        (&node.as_class_variable_and_write_node().unwrap()        ),
            Node::ClassVariableOperatorWriteNode    { .. } => collect_child_targets_of_class_variable_operator_write_node   (&node.as_class_variable_operator_write_node().unwrap()   ),
            Node::ClassVariableOrWriteNode          { .. } => collect_child_targets_of_class_variable_or_write_node         (&node.as_class_variable_or_write_node().unwrap()         ),
            Node::ClassVariableReadNode             { .. } => collect_child_targets_of_class_variable_read_node             (&node.as_class_variable_read_node().unwrap()             ),
            Node::ClassVariableTargetNode           { .. } => collect_child_targets_of_class_variable_target_node           (&node.as_class_variable_target_node().unwrap()           ),
            Node::ClassVariableWriteNode            { .. } => collect_child_targets_of_class_variable_write_node            (&node.as_class_variable_write_node().unwrap()            ),
            Node::ConstantAndWriteNode              { .. } => collect_child_targets_of_constant_and_write_node              (&node.as_constant_and_write_node().unwrap()              ),
            Node::ConstantOperatorWriteNode         { .. } => collect_child_targets_of_constant_operator_write_node         (&node.as_constant_operator_write_node().unwrap()         ),
            Node::ConstantOrWriteNode               { .. } => collect_child_targets_of_constant_or_write_node               (&node.as_constant_or_write_node().unwrap()               ),
            Node::ConstantPathAndWriteNode          { .. } => collect_child_targets_of_constant_path_and_write_node         (&node.as_constant_path_and_write_node().unwrap()         ),
            Node::ConstantPathNode                  { .. } => collect_child_targets_of_constant_path_node                   (&node.as_constant_path_node().unwrap()                   ),
            Node::ConstantPathOperatorWriteNode     { .. } => collect_child_targets_of_constant_path_operator_write_node    (&node.as_constant_path_operator_write_node().unwrap()    ),
            Node::ConstantPathOrWriteNode           { .. } => collect_child_targets_of_constant_path_or_write_node          (&node.as_constant_path_or_write_node().unwrap()          ),
            Node::ConstantPathTargetNode            { .. } => collect_child_targets_of_constant_path_target_node            (&node.as_constant_path_target_node().unwrap()            ),
            Node::ConstantPathWriteNode             { .. } => collect_child_targets_of_constant_path_write_node             (&node.as_constant_path_write_node().unwrap()             ),
            Node::ConstantReadNode                  { .. } => collect_child_targets_of_constant_read_node                   (&node.as_constant_read_node().unwrap()                   ),
            Node::ConstantTargetNode                { .. } => collect_child_targets_of_constant_target_node                 (&node.as_constant_target_node().unwrap()                 ),
            Node::ConstantWriteNode                 { .. } => collect_child_targets_of_constant_write_node                  (&node.as_constant_write_node().unwrap()                  ),
            Node::DefNode                           { .. } => collect_child_targets_of_def_node                             (&node.as_def_node().unwrap()                             ),
            Node::DefinedNode                       { .. } => collect_child_targets_of_defined_node                         (&node.as_defined_node().unwrap()                         ),
            Node::ElseNode                          { .. } => collect_child_targets_of_else_node                            (&node.as_else_node().unwrap()                            ),
            Node::EmbeddedStatementsNode            { .. } => collect_child_targets_of_embedded_statements_node             (&node.as_embedded_statements_node().unwrap()             ),
            Node::EmbeddedVariableNode              { .. } => collect_child_targets_of_embedded_variable_node               (&node.as_embedded_variable_node().unwrap()               ),
            Node::EnsureNode                        { .. } => collect_child_targets_of_ensure_node                          (&node.as_ensure_node().unwrap()                          ),
            Node::FalseNode                         { .. } => collect_child_targets_of_false_node                           (&node.as_false_node().unwrap()                           ),
            Node::FindPatternNode                   { .. } => collect_child_targets_of_find_pattern_node                    (&node.as_find_pattern_node().unwrap()                    ),
            Node::FlipFlopNode                      { .. } => collect_child_targets_of_flip_flop_node                       (&node.as_flip_flop_node().unwrap()                       ),
            Node::FloatNode                         { .. } => collect_child_targets_of_float_node                           (&node.as_float_node().unwrap()                           ),
            Node::ForNode                           { .. } => collect_child_targets_of_for_node                             (&node.as_for_node().unwrap()                             ),
            Node::ForwardingArgumentsNode           { .. } => collect_child_targets_of_forwarding_arguments_node            (&node.as_forwarding_arguments_node().unwrap()            ),
            Node::ForwardingParameterNode           { .. } => collect_child_targets_of_forwarding_parameter_node            (&node.as_forwarding_parameter_node().unwrap()            ),
            Node::ForwardingSuperNode               { .. } => collect_child_targets_of_forwarding_super_node                (&node.as_forwarding_super_node().unwrap()                ),
            Node::GlobalVariableAndWriteNode        { .. } => collect_child_targets_of_global_variable_and_write_node       (&node.as_global_variable_and_write_node().unwrap()       ),
            Node::GlobalVariableOperatorWriteNode   { .. } => collect_child_targets_of_global_variable_operator_write_node  (&node.as_global_variable_operator_write_node().unwrap()  ),
            Node::GlobalVariableOrWriteNode         { .. } => collect_child_targets_of_global_variable_or_write_node        (&node.as_global_variable_or_write_node().unwrap()        ),
            Node::GlobalVariableReadNode            { .. } => collect_child_targets_of_global_variable_read_node            (&node.as_global_variable_read_node().unwrap()            ),
            Node::GlobalVariableTargetNode          { .. } => collect_child_targets_of_global_variable_target_node          (&node.as_global_variable_target_node().unwrap()          ),
            Node::GlobalVariableWriteNode           { .. } => collect_child_targets_of_global_variable_write_node           (&node.as_global_variable_write_node().unwrap()           ),
            Node::HashNode                          { .. } => collect_child_targets_of_hash_node                            (&node.as_hash_node().unwrap()                            ),
            Node::HashPatternNode                   { .. } => collect_child_targets_of_hash_pattern_node                    (&node.as_hash_pattern_node().unwrap()                    ),
            Node::IfNode                            { .. } => collect_child_targets_of_if_node                              (&node.as_if_node().unwrap()                              ),
            Node::ImaginaryNode                     { .. } => collect_child_targets_of_imaginary_node                       (&node.as_imaginary_node().unwrap()                       ),
            Node::ImplicitNode                      { .. } => collect_child_targets_of_implicit_node                        (&node.as_implicit_node().unwrap()                        ),
            Node::ImplicitRestNode                  { .. } => collect_child_targets_of_implicit_rest_node                   (&node.as_implicit_rest_node().unwrap()                   ),
            Node::InNode                            { .. } => collect_child_targets_of_in_node                              (&node.as_in_node().unwrap()                              ),
            Node::IndexAndWriteNode                 { .. } => collect_child_targets_of_index_and_write_node                 (&node.as_index_and_write_node().unwrap()                 ),
            Node::IndexOperatorWriteNode            { .. } => collect_child_targets_of_index_operator_write_node            (&node.as_index_operator_write_node().unwrap()            ),
            Node::IndexOrWriteNode                  { .. } => collect_child_targets_of_index_or_write_node                  (&node.as_index_or_write_node().unwrap()                  ),
            Node::IndexTargetNode                   { .. } => collect_child_targets_of_index_target_node                    (&node.as_index_target_node().unwrap()                    ),
            Node::InstanceVariableAndWriteNode      { .. } => collect_child_targets_of_instance_variable_and_write_node     (&node.as_instance_variable_and_write_node().unwrap()     ),
            Node::InstanceVariableOperatorWriteNode { .. } => collect_child_targets_of_instance_variable_operator_write_node(&node.as_instance_variable_operator_write_node().unwrap()),
            Node::InstanceVariableOrWriteNode       { .. } => collect_child_targets_of_instance_variable_or_write_node      (&node.as_instance_variable_or_write_node().unwrap()      ),
            Node::InstanceVariableReadNode          { .. } => collect_child_targets_of_instance_variable_read_node          (&node.as_instance_variable_read_node().unwrap()          ),
            Node::InstanceVariableTargetNode        { .. } => collect_child_targets_of_instance_variable_target_node        (&node.as_instance_variable_target_node().unwrap()        ),
            Node::InstanceVariableWriteNode         { .. } => collect_child_targets_of_instance_variable_write_node         (&node.as_instance_variable_write_node().unwrap()         ),
            Node::IntegerNode                       { .. } => collect_child_targets_of_integer_node                         (&node.as_integer_node().unwrap()                         ),
            Node::InterpolatedMatchLastLineNode     { .. } => collect_child_targets_of_interpolated_match_last_line_node    (&node.as_interpolated_match_last_line_node().unwrap()    ),
            Node::InterpolatedRegularExpressionNode { .. } => collect_child_targets_of_interpolated_regular_expression_node (&node.as_interpolated_regular_expression_node().unwrap() ),
            Node::InterpolatedStringNode            { .. } => collect_child_targets_of_interpolated_string_node             (&node.as_interpolated_string_node().unwrap()             ),
            Node::InterpolatedSymbolNode            { .. } => collect_child_targets_of_interpolated_symbol_node             (&node.as_interpolated_symbol_node().unwrap()             ),
            Node::InterpolatedXStringNode           { .. } => collect_child_targets_of_interpolated_x_string_node           (&node.as_interpolated_x_string_node().unwrap()           ),
            Node::ItLocalVariableReadNode           { .. } => collect_child_targets_of_it_local_variable_read_node          (&node.as_it_local_variable_read_node().unwrap()          ),
            Node::ItParametersNode                  { .. } => collect_child_targets_of_it_parameters_node                   (&node.as_it_parameters_node().unwrap()                   ),
            Node::KeywordHashNode                   { .. } => collect_child_targets_of_keyword_hash_node                    (&node.as_keyword_hash_node().unwrap()                    ),
            Node::KeywordRestParameterNode          { .. } => collect_child_targets_of_keyword_rest_parameter_node          (&node.as_keyword_rest_parameter_node().unwrap()          ),
            Node::LambdaNode                        { .. } => collect_child_targets_of_lambda_node                          (&node.as_lambda_node().unwrap()                          ),
            Node::LocalVariableAndWriteNode         { .. } => collect_child_targets_of_local_variable_and_write_node        (&node.as_local_variable_and_write_node().unwrap()        ),
            Node::LocalVariableOperatorWriteNode    { .. } => collect_child_targets_of_local_variable_operator_write_node   (&node.as_local_variable_operator_write_node().unwrap()   ),
            Node::LocalVariableOrWriteNode          { .. } => collect_child_targets_of_local_variable_or_write_node         (&node.as_local_variable_or_write_node().unwrap()         ),
            Node::LocalVariableReadNode             { .. } => collect_child_targets_of_local_variable_read_node             (&node.as_local_variable_read_node().unwrap()             ),
            Node::LocalVariableTargetNode           { .. } => collect_child_targets_of_local_variable_target_node           (&node.as_local_variable_target_node().unwrap()           ),
            Node::LocalVariableWriteNode            { .. } => collect_child_targets_of_local_variable_write_node            (&node.as_local_variable_write_node().unwrap()            ),
            Node::MatchLastLineNode                 { .. } => collect_child_targets_of_match_last_line_node                 (&node.as_match_last_line_node().unwrap()                 ),
            Node::MatchPredicateNode                { .. } => collect_child_targets_of_match_predicate_node                 (&node.as_match_predicate_node().unwrap()                 ),
            Node::MatchRequiredNode                 { .. } => collect_child_targets_of_match_required_node                  (&node.as_match_required_node().unwrap()                  ),
            Node::MatchWriteNode                    { .. } => collect_child_targets_of_match_write_node                     (&node.as_match_write_node().unwrap()                     ),
            Node::MissingNode                       { .. } => collect_child_targets_of_missing_node                         (&node.as_missing_node().unwrap()                         ),
            Node::ModuleNode                        { .. } => collect_child_targets_of_module_node                          (&node.as_module_node().unwrap()                          ),
            Node::MultiTargetNode                   { .. } => collect_child_targets_of_multi_target_node                    (&node.as_multi_target_node().unwrap()                    ),
            Node::MultiWriteNode                    { .. } => collect_child_targets_of_multi_write_node                     (&node.as_multi_write_node().unwrap()                     ),
            Node::NextNode                          { .. } => collect_child_targets_of_next_node                            (&node.as_next_node().unwrap()                            ),
            Node::NilNode                           { .. } => collect_child_targets_of_nil_node                             (&node.as_nil_node().unwrap()                             ),
            Node::NoKeywordsParameterNode           { .. } => collect_child_targets_of_no_keywords_parameter_node           (&node.as_no_keywords_parameter_node().unwrap()           ),
            Node::NumberedParametersNode            { .. } => collect_child_targets_of_numbered_parameters_node             (&node.as_numbered_parameters_node().unwrap()             ),
            Node::NumberedReferenceReadNode         { .. } => collect_child_targets_of_numbered_reference_read_node         (&node.as_numbered_reference_read_node().unwrap()         ),
            Node::OptionalKeywordParameterNode      { .. } => collect_child_targets_of_optional_keyword_parameter_node      (&node.as_optional_keyword_parameter_node().unwrap()      ),
            Node::OptionalParameterNode             { .. } => collect_child_targets_of_optional_parameter_node              (&node.as_optional_parameter_node().unwrap()              ),
            Node::OrNode                            { .. } => collect_child_targets_of_or_node                              (&node.as_or_node().unwrap()                              ),
            Node::ParametersNode                    { .. } => collect_child_targets_of_parameters_node                      (&node.as_parameters_node().unwrap()                      ),
            Node::ParenthesesNode                   { .. } => collect_child_targets_of_parentheses_node                     (&node.as_parentheses_node().unwrap()                     ),
            Node::PinnedExpressionNode              { .. } => collect_child_targets_of_pinned_expression_node               (&node.as_pinned_expression_node().unwrap()               ),
            Node::PinnedVariableNode                { .. } => collect_child_targets_of_pinned_variable_node                 (&node.as_pinned_variable_node().unwrap()                 ),
            Node::PostExecutionNode                 { .. } => collect_child_targets_of_post_execution_node                  (&node.as_post_execution_node().unwrap()                  ),
            Node::PreExecutionNode                  { .. } => collect_child_targets_of_pre_execution_node                   (&node.as_pre_execution_node().unwrap()                   ),
            Node::ProgramNode                       { .. } => collect_child_targets_of_program_node                         (&node.as_program_node().unwrap()                         ),
            Node::RangeNode                         { .. } => collect_child_targets_of_range_node                           (&node.as_range_node().unwrap()                           ),
            Node::RationalNode                      { .. } => collect_child_targets_of_rational_node                        (&node.as_rational_node().unwrap()                        ),
            Node::RedoNode                          { .. } => collect_child_targets_of_redo_node                            (&node.as_redo_node().unwrap()                            ),
            Node::RegularExpressionNode             { .. } => collect_child_targets_of_regular_expression_node              (&node.as_regular_expression_node().unwrap()              ),
            Node::RequiredKeywordParameterNode      { .. } => collect_child_targets_of_required_keyword_parameter_node      (&node.as_required_keyword_parameter_node().unwrap()      ),
            Node::RequiredParameterNode             { .. } => collect_child_targets_of_required_parameter_node              (&node.as_required_parameter_node().unwrap()              ),
            Node::RescueModifierNode                { .. } => collect_child_targets_of_rescue_modifier_node                 (&node.as_rescue_modifier_node().unwrap()                 ),
            Node::RescueNode                        { .. } => collect_child_targets_of_rescue_node                          (&node.as_rescue_node().unwrap()                          ),
            Node::RestParameterNode                 { .. } => collect_child_targets_of_rest_parameter_node                  (&node.as_rest_parameter_node().unwrap()                  ),
            Node::RetryNode                         { .. } => collect_child_targets_of_retry_node                           (&node.as_retry_node().unwrap()                           ),
            Node::ReturnNode                        { .. } => collect_child_targets_of_return_node                          (&node.as_return_node().unwrap()                          ),
            Node::SelfNode                          { .. } => collect_child_targets_of_self_node                            (&node.as_self_node().unwrap()                            ),
            Node::ShareableConstantNode             { .. } => collect_child_targets_of_shareable_constant_node              (&node.as_shareable_constant_node().unwrap()              ),
            Node::SingletonClassNode                { .. } => collect_child_targets_of_singleton_class_node                 (&node.as_singleton_class_node().unwrap()                 ),
            Node::SourceEncodingNode                { .. } => collect_child_targets_of_source_encoding_node                 (&node.as_source_encoding_node().unwrap()                 ),
            Node::SourceFileNode                    { .. } => collect_child_targets_of_source_file_node                     (&node.as_source_file_node().unwrap()                     ),
            Node::SourceLineNode                    { .. } => collect_child_targets_of_source_line_node                     (&node.as_source_line_node().unwrap()                     ),
            Node::SplatNode                         { .. } => collect_child_targets_of_splat_node                           (&node.as_splat_node().unwrap()                           ),
            Node::StatementsNode                    { .. } => collect_child_targets_of_statements_node                      (&node.as_statements_node().unwrap()                      ),
            Node::StringNode                        { .. } => collect_child_targets_of_string_node                          (&node.as_string_node().unwrap()                          ),
            Node::SuperNode                         { .. } => collect_child_targets_of_super_node                           (&node.as_super_node().unwrap()                           ),
            Node::SymbolNode                        { .. } => collect_child_targets_of_symbol_node                          (&node.as_symbol_node().unwrap()                          ),
            Node::TrueNode                          { .. } => collect_child_targets_of_true_node                            (&node.as_true_node().unwrap()                            ),
            Node::UndefNode                         { .. } => collect_child_targets_of_undef_node                           (&node.as_undef_node().unwrap()                           ),
            Node::UnlessNode                        { .. } => collect_child_targets_of_unless_node                          (&node.as_unless_node().unwrap()                          ),
            Node::UntilNode                         { .. } => collect_child_targets_of_until_node                           (&node.as_until_node().unwrap()                           ),
            Node::WhenNode                          { .. } => collect_child_targets_of_when_node                            (&node.as_when_node().unwrap()                            ),
            Node::WhileNode                         { .. } => collect_child_targets_of_while_node                           (&node.as_while_node().unwrap()                           ),
            Node::XStringNode                       { .. } => collect_child_targets_of_x_string_node                        (&node.as_x_string_node().unwrap()                        ),
            Node::YieldNode                         { .. } => collect_child_targets_of_yield_node                           (&node.as_yield_node().unwrap()                           ),
        }
    }
}
fn collect_child_targets_of_alias_global_variable_node<'sh>(
    node: &AliasGlobalVariableNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.new_name()), &mut targets);
    push_node_regular(Some(node.old_name()), &mut targets);
    push_loc_regular(Some(node.keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_alias_method_node<'sh>(node: &AliasMethodNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.new_name()), &mut targets);
    push_node_regular(Some(node.old_name()), &mut targets);
    push_loc_regular(Some(node.keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_alternation_pattern_node<'sh>(
    node: &AlternationPatternNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.left()), &mut targets);
    push_node_regular(Some(node.right()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_and_node<'sh>(node: &AndNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.left()), &mut targets);
    push_node_regular(Some(node.right()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_arguments_node<'sh>(node: &ArgumentsNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_nodelist_regular(Some(node.arguments()), &mut targets);
    targets
}
fn collect_child_targets_of_array_node<'sh>(node: &ArrayNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_nodelist_regular(Some(node.elements()), &mut targets);
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_array_pattern_node<'sh>(node: &ArrayPatternNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.constant(), &mut targets);
    push_nodelist_regular(Some(node.requireds()), &mut targets);
    push_node_regular(node.rest(), &mut targets);
    push_nodelist_regular(Some(node.posts()), &mut targets);
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_assoc_node<'sh>(node: &AssocNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.key()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    push_loc_regular(node.operator_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_assoc_splat_node<'sh>(node: &AssocSplatNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.value(), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_back_reference_read_node<'sh>(
    node: &BackReferenceReadNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_begin_node<'sh>(node: &BeginNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(node.begin_keyword_loc(), &mut targets);
    push_node_regular(node.statements().map(|stmts| stmts.as_node()), &mut targets);
    push_node_opening_and_closing(node.rescue_clause().map(|r| r.as_node()), &mut targets);
    push_node_opening_and_closing(node.else_clause().map(|e| e.as_node()), &mut targets);
    push_node_opening_and_closing(node.ensure_clause().map(|e| e.as_node()), &mut targets);
    push_loc_closing(node.end_keyword_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_block_argument_node<'sh>(node: &BlockArgumentNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(node.expression(), &mut targets);
    targets
}
fn collect_child_targets_of_block_local_variable_node<'sh>(
    node: &BlockLocalVariableNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_block_node<'sh>(node: &BlockNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.parameters(), &mut targets);
    push_node_regular(node.body(), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_block_parameter_node<'sh>(node: &BlockParameterNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(node.name_loc(), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_block_parameters_node<'sh>(node: &BlockParametersNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.parameters().map(|node| node.as_node()), &mut targets);
    push_nodelist_regular(Some(node.locals()), &mut targets);
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}

fn collect_child_targets_of_break_node<'sh>(node: &BreakNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.arguments().map(|node| node.as_node()), &mut targets);
    push_loc_regular(Some(node.keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_call_and_write_node<'sh>(node: &CallAndWriteNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.receiver(), &mut targets);
    push_loc_regular(node.call_operator_loc(), &mut targets);
    push_loc_regular(node.message_loc(), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_call_node<'sh>(node: &CallNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.receiver(), &mut targets);
    push_loc_regular(node.call_operator_loc(), &mut targets);
    push_loc_regular(node.message_loc(), &mut targets);
    push_loc_opening(node.opening_loc(), &mut targets);
    push_node_regular(node.arguments().map(|node| node.as_node()), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    push_node_regular(node.block(), &mut targets);
    targets
}
fn collect_child_targets_of_call_operator_write_node<'sh>(
    node: &CallOperatorWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.receiver(), &mut targets);
    push_loc_regular(node.call_operator_loc(), &mut targets);
    push_loc_regular(node.message_loc(), &mut targets);
    push_loc_regular(Some(node.binary_operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_call_or_write_node<'sh>(node: &CallOrWriteNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.receiver(), &mut targets);
    push_loc_regular(node.call_operator_loc(), &mut targets);
    push_loc_regular(node.message_loc(), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_call_target_node<'sh>(node: &CallTargetNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.receiver()), &mut targets);
    push_loc_regular(Some(node.call_operator_loc()), &mut targets);
    push_loc_regular(Some(node.message_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_capture_pattern_node<'sh>(node: &CapturePatternNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.value()), &mut targets);
    push_node_regular(Some(node.target().as_node()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_case_match_node<'sh>(node: &CaseMatchNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_opening_like(node.predicate(), &mut targets);
    push_nodelist_opening_and_closing(Some(node.conditions()), &mut targets);
    push_node_opening_and_closing(node.else_clause().map(|e| e.as_node()), &mut targets);
    push_loc_opening(Some(node.case_keyword_loc()), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_case_node<'sh>(node: &CaseNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_opening_like(node.predicate(), &mut targets);
    push_nodelist_opening_and_closing(Some(node.conditions()), &mut targets);
    push_node_opening_and_closing(node.else_clause().map(|e| e.as_node()), &mut targets);
    push_loc_opening(Some(node.case_keyword_loc()), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_class_node<'sh>(node: &ClassNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.class_keyword_loc()), &mut targets);
    push_node_opening_like(Some(node.constant_path()), &mut targets);
    push_loc_regular(node.inheritance_operator_loc(), &mut targets);
    push_node_opening_like(node.superclass(), &mut targets);
    push_node_regular(node.body(), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_class_variable_and_write_node<'sh>(
    node: &ClassVariableAndWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_class_variable_operator_write_node<'sh>(
    node: &ClassVariableOperatorWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.binary_operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_class_variable_or_write_node<'sh>(
    node: &ClassVariableOrWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_class_variable_read_node<'sh>(
    node: &ClassVariableReadNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_class_variable_target_node<'sh>(
    node: &ClassVariableTargetNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_class_variable_write_node<'sh>(
    node: &ClassVariableWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_constant_and_write_node<'sh>(node: &ConstantAndWriteNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_constant_operator_write_node<'sh>(
    node: &ConstantOperatorWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.binary_operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_constant_or_write_node<'sh>(node: &ConstantOrWriteNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_constant_path_and_write_node<'sh>(
    node: &ConstantPathAndWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.target().as_node()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_constant_path_node<'sh>(node: &ConstantPathNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.parent(), &mut targets);
    push_loc_regular(Some(node.delimiter_loc()), &mut targets);
    push_loc_regular(Some(node.name_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_constant_path_operator_write_node<'sh>(
    node: &ConstantPathOperatorWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.target().as_node()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    push_loc_regular(Some(node.binary_operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_constant_path_or_write_node<'sh>(
    node: &ConstantPathOrWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.target().as_node()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_constant_path_target_node<'sh>(
    node: &ConstantPathTargetNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.parent(), &mut targets);
    push_loc_regular(Some(node.delimiter_loc()), &mut targets);
    push_loc_regular(Some(node.name_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_constant_path_write_node<'sh>(
    node: &ConstantPathWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.target().as_node()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_constant_read_node<'sh>(node: &ConstantReadNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_constant_target_node<'sh>(node: &ConstantTargetNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_constant_write_node<'sh>(node: &ConstantWriteNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_def_node<'sh>(node: &DefNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening_like(Some(node.name_loc()), &mut targets);
    push_node_regular(node.receiver(), &mut targets);
    push_node_regular(node.parameters().map(|p| p.as_node()), &mut targets);
    push_node_regular(node.body(), &mut targets);
    push_loc_opening(Some(node.def_keyword_loc()), &mut targets);
    push_loc_regular(node.operator_loc(), &mut targets);
    push_loc_opening(node.lparen_loc(), &mut targets);
    push_loc_opening_and_closing(node.rparen_loc(), &mut targets);
    push_loc_regular(node.equal_loc(), &mut targets);
    push_loc_closing(node.end_keyword_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_defined_node<'sh>(node: &DefinedNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(node.lparen_loc(), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    push_loc_closing(node.rparen_loc(), &mut targets);
    push_loc_regular(Some(node.keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_else_node<'sh>(node: &ElseNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening_and_closing(Some(node.else_keyword_loc()), &mut targets);
    push_node_regular(node.statements().map(|s| s.as_node()), &mut targets);
    push_loc_opening_and_closing(node.end_keyword_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_embedded_statements_node<'sh>(
    node: &EmbeddedStatementsNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_node_regular(node.statements().map(|s| s.as_node()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_embedded_variable_node<'sh>(node: &EmbeddedVariableNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.variable()), &mut targets);
    targets
}
fn collect_child_targets_of_ensure_node<'sh>(node: &EnsureNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening_and_closing(Some(node.ensure_keyword_loc()), &mut targets);
    push_node_regular(node.statements().map(|s| s.as_node()), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_false_node<'sh>(node: &FalseNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_find_pattern_node<'sh>(node: &FindPatternNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.constant(), &mut targets);
    push_node_regular(Some(node.left().as_node()), &mut targets);
    push_nodelist_regular(Some(node.requireds()), &mut targets);
    push_node_regular(Some(node.right()), &mut targets);
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_flip_flop_node<'sh>(node: &FlipFlopNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.left(), &mut targets);
    push_node_regular(node.right(), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_float_node<'sh>(node: &FloatNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_for_node<'sh>(node: &ForNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.index()), &mut targets);
    push_node_regular(Some(node.collection()), &mut targets);
    push_node_regular(node.statements().map(|s| s.as_node()), &mut targets);
    push_loc_opening(Some(node.for_keyword_loc()), &mut targets);
    push_loc_regular(Some(node.in_keyword_loc()), &mut targets);
    push_loc_regular(node.do_keyword_loc(), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_forwarding_arguments_node<'sh>(
    node: &ForwardingArgumentsNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_forwarding_parameter_node<'sh>(
    node: &ForwardingParameterNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_forwarding_super_node<'sh>(node: &ForwardingSuperNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.block().map(|b| b.as_node()), &mut targets);
    targets
}
fn collect_child_targets_of_global_variable_and_write_node<'sh>(
    node: &GlobalVariableAndWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_global_variable_operator_write_node<'sh>(
    node: &GlobalVariableOperatorWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.binary_operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_global_variable_or_write_node<'sh>(
    node: &GlobalVariableOrWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_global_variable_read_node<'sh>(
    node: &GlobalVariableReadNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_global_variable_target_node<'sh>(
    node: &GlobalVariableTargetNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_global_variable_write_node<'sh>(
    node: &GlobalVariableWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_hash_node<'sh>(node: &HashNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    push_nodelist_regular(Some(node.elements()), &mut targets);
    targets
}
fn collect_child_targets_of_hash_pattern_node<'sh>(node: &HashPatternNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.constant(), &mut targets);
    push_nodelist_regular(Some(node.elements()), &mut targets);
    push_node_regular(node.rest(), &mut targets);
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_if_node<'sh>(node: &IfNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(node.if_keyword_loc(), &mut targets);
    push_node_regular(Some(node.predicate()), &mut targets);
    push_loc_regular(node.then_keyword_loc(), &mut targets);
    push_node_regular(node.statements().map(|s| s.as_node()), &mut targets);
    push_node_regular(node.subsequent(), &mut targets);
    push_loc_closing(node.end_keyword_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_imaginary_node<'sh>(node: &ImaginaryNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.numeric()), &mut targets);
    targets
}
fn collect_child_targets_of_implicit_node<'sh>(node: &ImplicitNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_implicit_rest_node<'sh>(node: &ImplicitRestNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_in_node<'sh>(node: &InNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_opening_like(Some(node.pattern()), &mut targets);
    push_nodelist_regular(node.statements().map(|s| s.body()), &mut targets);
    push_loc_opening_and_closing(Some(node.in_loc()), &mut targets);
    push_loc_opening_like(node.then_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_index_and_write_node<'sh>(node: &IndexAndWriteNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.receiver(), &mut targets);
    push_loc_regular(node.call_operator_loc(), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_node_regular(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    push_node_regular(node.block().map(|b| b.as_node()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_index_operator_write_node<'sh>(
    node: &IndexOperatorWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.receiver(), &mut targets);
    push_loc_regular(node.call_operator_loc(), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_node_regular(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    push_node_regular(node.block().map(|b| b.as_node()), &mut targets);
    push_loc_regular(Some(node.binary_operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_index_or_write_node<'sh>(node: &IndexOrWriteNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.receiver(), &mut targets);
    push_loc_regular(node.call_operator_loc(), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_node_regular(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    push_node_regular(node.block().map(|b| b.as_node()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_index_target_node<'sh>(node: &IndexTargetNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.receiver()), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_node_regular(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    push_node_regular(node.block().map(|b| b.as_node()), &mut targets);
    targets
}
fn collect_child_targets_of_instance_variable_and_write_node<'sh>(
    node: &InstanceVariableAndWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_instance_variable_operator_write_node<'sh>(
    node: &InstanceVariableOperatorWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.binary_operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_instance_variable_or_write_node<'sh>(
    node: &InstanceVariableOrWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_instance_variable_read_node<'sh>(
    node: &InstanceVariableReadNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_instance_variable_target_node<'sh>(
    node: &InstanceVariableTargetNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_instance_variable_write_node<'sh>(
    node: &InstanceVariableWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_integer_node<'sh>(node: &IntegerNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_interpolated_match_last_line_node<'sh>(
    node: &InterpolatedMatchLastLineNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_nodelist_regular(Some(node.parts()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_interpolated_regular_expression_node<'sh>(
    node: &InterpolatedRegularExpressionNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.opening_loc()), &mut targets);
    push_nodelist_regular(Some(node.parts()), &mut targets);
    push_loc_regular(Some(node.closing_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_interpolated_string_node<'sh>(
    node: &InterpolatedStringNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(node.opening_loc(), &mut targets);
    push_nodelist_regular(Some(node.parts()), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_interpolated_symbol_node<'sh>(
    node: &InterpolatedSymbolNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(node.opening_loc(), &mut targets);
    push_nodelist_regular(Some(node.parts()), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_interpolated_x_string_node<'sh>(
    node: &InterpolatedXStringNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_nodelist_regular(Some(node.parts()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_it_local_variable_read_node<'sh>(
    node: &ItLocalVariableReadNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_it_parameters_node<'sh>(node: &ItParametersNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_keyword_hash_node<'sh>(node: &KeywordHashNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_nodelist_regular(Some(node.elements()), &mut targets);
    targets
}
fn collect_child_targets_of_keyword_rest_parameter_node<'sh>(
    node: &KeywordRestParameterNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(node.name_loc(), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_lambda_node<'sh>(node: &LambdaNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    push_node_regular(node.parameters(), &mut targets);
    push_node_regular(node.body(), &mut targets);
    targets
}
fn collect_child_targets_of_local_variable_and_write_node<'sh>(
    node: &LocalVariableAndWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_local_variable_operator_write_node<'sh>(
    node: &LocalVariableOperatorWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.binary_operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_local_variable_or_write_node<'sh>(
    node: &LocalVariableOrWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_local_variable_read_node<'sh>(
    node: &LocalVariableReadNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_local_variable_target_node<'sh>(
    node: &LocalVariableTargetNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_local_variable_write_node<'sh>(
    node: &LocalVariableWriteNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_match_last_line_node<'sh>(node: &MatchLastLineNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_regular(Some(node.content_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_match_predicate_node<'sh>(node: &MatchPredicateNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.value()), &mut targets);
    push_node_regular(Some(node.pattern()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_match_required_node<'sh>(node: &MatchRequiredNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.value()), &mut targets);
    push_node_regular(Some(node.pattern()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_match_write_node<'sh>(node: &MatchWriteNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.call().as_node()), &mut targets);
    push_nodelist_regular(Some(node.targets()), &mut targets);
    targets
}
fn collect_child_targets_of_missing_node<'sh>(node: &MissingNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_module_node<'sh>(node: &ModuleNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.module_keyword_loc()), &mut targets);
    push_node_regular(Some(node.constant_path()), &mut targets);
    push_node_regular(node.body(), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_multi_target_node<'sh>(node: &MultiTargetNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_nodelist_regular(Some(node.lefts()), &mut targets);
    push_node_regular(node.rest(), &mut targets);
    push_nodelist_regular(Some(node.rights()), &mut targets);
    push_loc_opening(node.lparen_loc(), &mut targets);
    push_loc_closing(node.rparen_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_multi_write_node<'sh>(node: &MultiWriteNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_nodelist_regular(Some(node.lefts()), &mut targets);
    push_node_regular(node.rest(), &mut targets);
    push_nodelist_regular(Some(node.rights()), &mut targets);
    push_loc_opening(node.lparen_loc(), &mut targets);
    push_loc_closing(node.rparen_loc(), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_next_node<'sh>(node: &NextNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc_regular(Some(node.keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_nil_node<'sh>(node: &NilNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_no_keywords_parameter_node<'sh>(
    node: &NoKeywordsParameterNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_loc_regular(Some(node.keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_numbered_parameters_node<'sh>(
    node: &NumberedParametersNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_numbered_reference_read_node<'sh>(
    node: &NumberedReferenceReadNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_optional_keyword_parameter_node<'sh>(
    node: &OptionalKeywordParameterNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_optional_parameter_node<'sh>(node: &OptionalParameterNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.value()), &mut targets);
    targets
}
fn collect_child_targets_of_or_node<'sh>(node: &OrNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.left()), &mut targets);
    push_node_regular(Some(node.right()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_parameters_node<'sh>(node: &ParametersNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_nodelist_regular(Some(node.requireds()), &mut targets);
    push_nodelist_regular(Some(node.optionals()), &mut targets);
    push_node_regular(node.rest(), &mut targets);
    push_nodelist_regular(Some(node.posts()), &mut targets);
    push_nodelist_regular(Some(node.keywords()), &mut targets);
    push_node_regular(node.keyword_rest(), &mut targets);
    push_node_regular(node.block().map(|b| b.as_node()), &mut targets);
    targets
}
fn collect_child_targets_of_parentheses_node<'sh>(node: &ParenthesesNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.body(), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_pinned_expression_node<'sh>(node: &PinnedExpressionNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.expression()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_loc_opening(Some(node.lparen_loc()), &mut targets);
    push_loc_closing(Some(node.rparen_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_pinned_variable_node<'sh>(node: &PinnedVariableNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.variable()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_post_execution_node<'sh>(node: &PostExecutionNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.statements().map(|s| s.as_node()), &mut targets);
    push_loc_regular(Some(node.keyword_loc()), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_pre_execution_node<'sh>(node: &PreExecutionNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.statements().map(|s| s.as_node()), &mut targets);
    push_loc_regular(Some(node.keyword_loc()), &mut targets);
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_program_node<'sh>(node: &ProgramNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.statements().as_node()), &mut targets);
    targets
}
fn collect_child_targets_of_range_node<'sh>(node: &RangeNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(node.left(), &mut targets);
    push_node_regular(node.right(), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_rational_node<'sh>(node: &RationalNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_redo_node<'sh>(node: &RedoNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_regular_expression_node<'sh>(node: &RegularExpressionNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_regular(Some(node.content_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_required_keyword_parameter_node<'sh>(
    node: &RequiredKeywordParameterNode<'sh>,
) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.name_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_required_parameter_node<'sh>(node: &RequiredParameterNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_rescue_modifier_node<'sh>(node: &RescueModifierNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.expression()), &mut targets);
    push_loc_regular(Some(node.keyword_loc()), &mut targets);
    push_node_regular(Some(node.rescue_expression()), &mut targets);
    targets
}
fn collect_child_targets_of_rescue_node<'sh>(node: &RescueNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening_and_closing(Some(node.keyword_loc()), &mut targets);
    push_nodelist_opening_like(Some(node.exceptions()), &mut targets);
    push_loc_opening_like(node.operator_loc(), &mut targets);
    push_node_opening_like(node.reference(), &mut targets);
    push_loc_opening_like(node.then_keyword_loc(), &mut targets);
    push_node_regular(node.statements().map(|s| s.as_node()), &mut targets);
    push_node_opening_and_closing(node.subsequent().map(|s| s.as_node()), &mut targets);
    targets
}
fn collect_child_targets_of_rest_parameter_node<'sh>(node: &RestParameterNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(node.name_loc(), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_retry_node<'sh>(node: &RetryNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_return_node<'sh>(node: &ReturnNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.keyword_loc()), &mut targets);
    push_node_regular(node.arguments().map(|a| a.as_node()), &mut targets);
    targets
}
fn collect_child_targets_of_self_node<'sh>(node: &SelfNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_shareable_constant_node<'sh>(node: &ShareableConstantNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_node_regular(Some(node.write()), &mut targets);
    targets
}
fn collect_child_targets_of_singleton_class_node<'sh>(node: &SingletonClassNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.class_keyword_loc()), &mut targets);
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(Some(node.expression()), &mut targets);
    push_node_regular(node.body(), &mut targets);
    push_loc_closing(Some(node.end_keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_source_encoding_node<'sh>(node: &SourceEncodingNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_source_file_node<'sh>(node: &SourceFileNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_source_line_node<'sh>(node: &SourceLineNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_splat_node<'sh>(node: &SplatNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.operator_loc()), &mut targets);
    push_node_regular(node.expression(), &mut targets);
    targets
}
fn collect_child_targets_of_statements_node<'sh>(node: &StatementsNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_nodelist_regular(Some(node.body()), &mut targets);
    targets
}
fn collect_child_targets_of_string_node<'sh>(node: &StringNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc_regular(Some(node.content_loc()), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_super_node<'sh>(node: &SuperNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.keyword_loc()), &mut targets);
    push_loc_opening(node.lparen_loc(), &mut targets);
    push_node_regular(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc_closing(node.rparen_loc(), &mut targets);
    push_node_regular(node.block(), &mut targets);
    targets
}
fn collect_child_targets_of_symbol_node<'sh>(node: &SymbolNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(node.opening_loc(), &mut targets);
    push_loc_regular(node.value_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_true_node<'sh>(node: &TrueNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_regular(Some(node.location()), &mut targets);
    targets
}
fn collect_child_targets_of_undef_node<'sh>(node: &UndefNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_nodelist_regular(Some(node.names()), &mut targets);
    push_loc_regular(Some(node.keyword_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_unless_node<'sh>(node: &UnlessNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.keyword_loc()), &mut targets);
    push_node_regular(Some(node.predicate()), &mut targets);
    push_loc_regular(node.then_keyword_loc(), &mut targets);
    push_node_regular(node.statements().map(|s| s.as_node()), &mut targets);
    push_node_regular(node.else_clause().map(|e| e.as_node()), &mut targets);
    push_loc_closing(node.end_keyword_loc(), &mut targets);
    targets
}
fn collect_child_targets_of_until_node<'sh>(node: &UntilNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.keyword_loc()), &mut targets);
    push_loc_regular(node.do_keyword_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    push_node_regular(Some(node.predicate()), &mut targets);
    push_node_regular(node.statements().map(|s| s.as_node()), &mut targets);
    targets
}
fn collect_child_targets_of_when_node<'sh>(node: &WhenNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening_and_closing(Some(node.keyword_loc()), &mut targets);
    push_nodelist_opening_like(Some(node.conditions()), &mut targets);
    push_loc_opening_like(node.then_keyword_loc(), &mut targets);
    push_node_regular(node.statements().map(|s| s.as_node()), &mut targets);
    targets
}
fn collect_child_targets_of_while_node<'sh>(node: &WhileNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.keyword_loc()), &mut targets);
    push_loc_regular(node.do_keyword_loc(), &mut targets);
    push_loc_closing(node.closing_loc(), &mut targets);
    push_node_regular(Some(node.predicate()), &mut targets);
    push_node_regular(node.statements().map(|s| s.as_node()), &mut targets);
    targets
}
fn collect_child_targets_of_x_string_node<'sh>(node: &XStringNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.opening_loc()), &mut targets);
    push_loc_regular(Some(node.content_loc()), &mut targets);
    push_loc_closing(Some(node.closing_loc()), &mut targets);
    targets
}
fn collect_child_targets_of_yield_node<'sh>(node: &YieldNode<'sh>) -> Vec<CommentTarget<'sh>> {
    let mut targets = Vec::new();
    push_loc_opening(Some(node.keyword_loc()), &mut targets);
    push_loc_opening(node.lparen_loc(), &mut targets);
    push_node_regular(node.arguments().map(|a| a.as_node()), &mut targets);
    push_loc_closing(node.rparen_loc(), &mut targets);
    targets
}

#[allow(dead_code)]
fn push_loc_opening<'sh>(loc: Option<Location<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match loc {
        Some(loc) => {
            targets.push(CommentTarget::from((loc, TargetType::Opening)));
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_loc_opening_like<'sh>(loc: Option<Location<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match loc {
        Some(loc) => {
            targets.push(CommentTarget::from((loc, TargetType::OpeningLike)));
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_loc_opening_and_closing<'sh>(loc: Option<Location<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match loc {
        Some(loc) => {
            targets.push(CommentTarget::from((loc, TargetType::OpeningAndClosing)));
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_loc_opening_like_and_closing<'sh>(loc: Option<Location<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match loc {
        Some(loc) => {
            targets.push(CommentTarget::from((loc, TargetType::OpeningLikeAndClosing)));
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_loc_closing<'sh>(loc: Option<Location<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match loc {
        Some(loc) => {
            targets.push(CommentTarget::from((loc, TargetType::Closing)));
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_loc_regular<'sh>(loc: Option<Location<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match loc {
        Some(loc) => {
            targets.push(CommentTarget::from((loc, TargetType::Regular)));
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_node_opening<'sh>(node: Option<Node<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match node {
        Some(node) => {
            targets.push(CommentTarget::from((node, TargetType::Opening)));
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_node_opening_like<'sh>(node: Option<Node<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match node {
        Some(node) => {
            targets.push(CommentTarget::from((node, TargetType::OpeningLike)));
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_node_opening_and_closing<'sh>(node: Option<Node<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match node {
        Some(node) => {
            targets.push(CommentTarget::from((node, TargetType::OpeningAndClosing)));
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_node_opening_like_and_closing<'sh>(node: Option<Node<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match node {
        Some(node) => {
            targets.push(CommentTarget::from((node, TargetType::OpeningLikeAndClosing)));
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_node_closing<'sh>(node: Option<Node<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match node {
        Some(node) => {
            targets.push(CommentTarget::from((node, TargetType::Closing)));
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_node_regular<'sh>(node: Option<Node<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match node {
        Some(node) => {
            targets.push(CommentTarget::Node(node, TargetType::Regular));
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_nodelist_opening<'sh>(nodelist: Option<NodeList<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match nodelist {
        Some(nodelist) => {
            for node in nodelist.iter() {
                targets.push(CommentTarget::from((node, TargetType::Opening)));
            }
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_nodelist_opening_like<'sh>(nodelist: Option<NodeList<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match nodelist {
        Some(nodelist) => {
            for node in nodelist.iter() {
                targets.push(CommentTarget::from((node, TargetType::OpeningLike)));
            }
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_nodelist_opening_and_closing<'sh>(nodelist: Option<NodeList<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match nodelist {
        Some(nodelist) => {
            for node in nodelist.iter() {
                targets.push(CommentTarget::from((node, TargetType::OpeningAndClosing)));
            }
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_nodelist_opening_like_and_closing<'sh>(nodelist: Option<NodeList<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match nodelist {
        Some(nodelist) => {
            for node in nodelist.iter() {
                targets.push(CommentTarget::from((node, TargetType::OpeningLikeAndClosing)));
            }
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_nodelist_closing<'sh>(nodelist: Option<NodeList<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match nodelist {
        Some(nodelist) => {
            for node in nodelist.iter() {
                targets.push(CommentTarget::from((node, TargetType::Closing)));
            }
        }
        None => {}
    }
}
#[allow(dead_code)]
fn push_nodelist_regular<'sh>(nodelist: Option<NodeList<'sh>>, targets: &mut Vec<CommentTarget<'sh>>) {
    match nodelist {
        Some(nodelist) => {
            for node in nodelist.iter() {
                targets.push(CommentTarget::Node(node, TargetType::Regular));
            }
        }
        None => {}
    }
}
