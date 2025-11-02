use crate::BuildContext;
use crate::builder::builder::{array, break_parent, hardline, line_suffix, string};
use crate::builder::prism::leading_line_breaks;
use crate::document::Document;
use ruby_prism::Comment;
use ruby_prism::CommentType;
use ruby_prism::Node;
use ruby_prism::Visit;

/// Metadata about comments used for advanced placement logic.
pub struct CommentMetadata {
    pub comment_start_offset: usize,
    pub preceding_node_location: Option<NodeLocation>,
    pub enclosing_node_location: Option<NodeLocation>,
    pub following_node_location: Option<NodeLocation>,
    pub placement: CommentPlacement,
}

/// Location information for a node.
#[derive(Clone, Copy, Debug)]
pub struct NodeLocation {
    pub start_offset: usize,
    pub end_offset: usize,
}

/// Enum representing comment placement types.
#[derive(Debug)]
pub enum CommentPlacement {
    EndOfLine,
    OwnLine,
    Remaining,
}

/// Visitor for collecting node locations.
struct NodeLocationCollector<'sh> {
    node_locations: &'sh mut Vec<NodeLocation>,
}

/// Implementation of the visitor pattern for collecting node locations.
impl<'sh> Visit<'sh> for NodeLocationCollector<'sh> {
    fn visit_branch_node_enter(&mut self, node: Node<'sh>) {
        self.node_locations.push(NodeLocation {
            start_offset: node.location().start_offset(),
            end_offset: node.location().end_offset(),
        });
    }
    fn visit_leaf_node_enter(&mut self, node: Node<'sh>) {
        self.node_locations.push(NodeLocation {
            start_offset: node.location().start_offset(),
            end_offset: node.location().end_offset(),
        });
    }
}

/// Collect node locations in sorted order.
pub fn collect_sorted_node_locations<'sh>(root: &Node<'sh>) -> Vec<NodeLocation> {
    let mut node_locations = Vec::new();
    let mut collector = NodeLocationCollector {
        node_locations: &mut node_locations,
    };
    collector.visit(root);
    node_locations.sort_unstable_by_key(|loc| loc.start_offset);
    node_locations
}

/// Decorates a comment with metadata for placement decisions.
pub fn decorate_comment<'sh>(
    comment: &'sh Comment<'sh>,
    sorted_node_locations: &'sh Vec<NodeLocation>,
    source: &'sh [u8],
) -> CommentMetadata {
    let comment_start_offset = comment.location().start_offset();
    let comment_end_offset = comment.location().end_offset();

    // Find preceding node - binary search for nodes ending before comment
    let preceding_node_location = find_preceding_node(sorted_node_locations, comment_start_offset);

    // Find following node - binary search for nodes starting after comment
    let following_node_location = find_following_node(sorted_node_locations, comment_end_offset);

    // Find enclosing node - needs to scan all candidates
    let enclosing_node_location = find_enclosing_node(
        sorted_node_locations,
        comment_start_offset,
        comment_end_offset,
    );

    let placement = determine_placement(
        comment,
        preceding_node_location.as_ref(),
        following_node_location.as_ref(),
        source,
    );

    CommentMetadata {
        comment_start_offset,
        preceding_node_location,
        enclosing_node_location,
        following_node_location,
        placement,
    }
}

/// Finds the preceding node (node_end <= comment_start).
/// Uses binary search to find candidates, then scans to find the best match.
/// Matches original logic: largest end_offset, then largest start_offset (less specific node).
fn find_preceding_node(
    sorted_node_locations: &[NodeLocation],
    comment_start: usize,
) -> Option<NodeLocation> {
    let mut result: Option<NodeLocation> = None;

    // Scan all nodes to find the best preceding node
    for node in sorted_node_locations.iter() {
        if node.end_offset <= comment_start {
            match result {
                Some(prev) => {
                    if node.end_offset > prev.end_offset {
                        // This node ends closer to the comment
                        result = Some(*node);
                    } else if node.end_offset == prev.end_offset {
                        // Same end_offset, prefer larger (less specific) node
                        // Original logic: prev_preceding.start_offset < node_start_offset
                        if prev.start_offset < node.start_offset {
                            result = Some(*node);
                        }
                    }
                }
                None => result = Some(*node),
            }
        }
    }
    result
}

/// Finds the following node (comment_end <= node_start).
/// Uses binary search to find the first node starting after comment.
fn find_following_node(
    sorted_node_locations: &[NodeLocation],
    comment_end: usize,
) -> Option<NodeLocation> {
    // Binary search for the first node that starts at or after comment_end
    let idx = sorted_node_locations.partition_point(|node| node.start_offset < comment_end);

    if idx >= sorted_node_locations.len() {
        return None;
    }

    let mut result = sorted_node_locations[idx];

    // Check subsequent nodes with the same start_offset
    // Prefer smaller (more specific) nodes
    for i in (idx + 1)..sorted_node_locations.len() {
        let node = sorted_node_locations[i];
        if node.start_offset != result.start_offset {
            break;
        }
        if node.end_offset < result.end_offset {
            result = node;
        }
    }

    Some(result)
}

/// Finds the smallest enclosing node (node_start <= comment_start <= comment_end <= node_end).
/// This requires checking all nodes, but we can optimize by using binary search for candidates.
fn find_enclosing_node(
    sorted_node_locations: &[NodeLocation],
    comment_start: usize,
    comment_end: usize,
) -> Option<NodeLocation> {
    // Binary search to find where nodes could start containing the comment
    let start_idx = sorted_node_locations.partition_point(|node| node.start_offset < comment_start);

    let mut result: Option<NodeLocation> = None;

    // Check nodes starting before or at comment_start
    // We need to go backwards from start_idx because nodes with earlier starts might enclose
    for i in (0..=start_idx.min(sorted_node_locations.len().saturating_sub(1))).rev() {
        if i >= sorted_node_locations.len() {
            continue;
        }
        let node = sorted_node_locations[i];

        // Node must start before or at comment start
        if node.start_offset > comment_start {
            continue;
        }

        // Node must end after or at comment end
        if node.end_offset < comment_end {
            break; // Earlier nodes will also not contain the comment
        }

        // This node encloses the comment
        if node.start_offset <= comment_start && comment_end <= node.end_offset {
            match result {
                Some(prev) => {
                    // Prefer smaller (more specific) enclosing node
                    let prev_size = prev.end_offset - prev.start_offset;
                    let curr_size = node.end_offset - node.start_offset;
                    if curr_size < prev_size {
                        result = Some(node);
                    }
                }
                None => result = Some(node),
            }
        }
    }

    result
}

/// Determines the placement of a comment based on surrounding nodes and source code.
pub fn determine_placement(
    comment: &Comment,
    preceding_node_location: Option<&NodeLocation>,
    following_node_location: Option<&NodeLocation>,
    source: &[u8],
) -> CommentPlacement {
    fn has_newline_in_range(source: &[u8], start_offset: usize, end_offset: usize) -> bool {
        let end = end_offset.min(source.len());
        source[start_offset..end].iter().any(|&b| b == b'\n')
    }
    let comment_start_offset = comment.location().start_offset();
    let comment_end_offset = comment.location().end_offset();
    let has_newline_before = preceding_node_location.map_or(true, |loc| {
        has_newline_in_range(source, loc.end_offset, comment_start_offset)
    });
    let has_newline_after = following_node_location.map_or(true, |loc| {
        // block comments contain \n at the end of the comment text, so we need to adjust the range accordingly
        // ``````
        // =begin
        // block comment
        // =end\n
        // ```
        match comment.type_() {
            CommentType::EmbDocComment => {
                has_newline_in_range(source, comment_end_offset - 1, loc.start_offset)
            }
            CommentType::InlineComment => {
                has_newline_in_range(source, comment_end_offset, loc.start_offset)
            }
        }
    });
    match (has_newline_before, has_newline_after) {
        (true, true) => CommentPlacement::OwnLine,
        (false, true) => CommentPlacement::EndOfLine,
        (_, _) => CommentPlacement::Remaining,
    }
}

/// Builds leading comments for a given node.
/// ```ruby
/// # leading comment
/// foo
/// ```
pub fn leading_comments(node: &Node, context: &mut BuildContext) -> Document {
    let mut documents = Vec::new();
    loop {
        match context.comments.peek() {
            Some(comment) => {
                let comment_start_offset = comment.location().start_offset();
                // comment is not before the node start
                if comment_start_offset >= node.location().start_offset() {
                    break;
                }
                let metadata = context.comment_metadata.get(&comment_start_offset);
                let is_leading = match metadata {
                    Some(metadata) => match metadata.placement {
                        CommentPlacement::OwnLine => {
                            metadata.following_node_location.map_or(true, |following| {
                                following.start_offset == node.location().start_offset()
                            })
                        }
                        CommentPlacement::EndOfLine => false,
                        CommentPlacement::Remaining => unimplemented!(
                            "remaining comments are not supported as leading comments"
                        ),
                    },
                    None => {
                        unimplemented!(
                            "comments without metadata are not supported as leading comments"
                        )
                    }
                };
                if !is_leading {
                    break;
                }
                documents.push(leading_line_breaks(context, comment_start_offset, 1usize));
                let comment = context.comments.next().unwrap();
                documents.push(build_comment(&comment));
                documents.push(hardline());
                context.built_end = comment.location().end_offset();
            }
            None => break,
        }
    }
    array(&documents)
}

/// Builds owning comments for a given node.
/// ```ruby
/// if foo then
///   # owning comment
/// end
/// ```
pub fn owning_comments(node: &Node, context: &mut BuildContext) -> Option<Document> {
    let mut documents = Vec::new();
    loop {
        match context.comments.peek() {
            Some(comment) => {
                let comment_start_offset = comment.location().start_offset();
                let node_start = node.location().start_offset();
                let node_end = node.location().end_offset();
                // comment is outside the node range
                if comment_start_offset < node_start || node_end < comment_start_offset {
                    break;
                }
                let metadata = context.comment_metadata.get(&comment_start_offset);
                let is_owning = match metadata {
                    Some(metadata) => match metadata.placement {
                        CommentPlacement::OwnLine => {
                            metadata.enclosing_node_location.map_or(true, |enclosing| {
                                enclosing.start_offset == node_start
                                    && enclosing.end_offset == node_end
                            })
                        }
                        CommentPlacement::EndOfLine => {
                            metadata.preceding_node_location.map_or(true, |preceding| {
                                preceding.start_offset != node_start
                                    || preceding.end_offset != node_end
                            })
                        }
                        CommentPlacement::Remaining => {
                            unimplemented!(
                                "remaining comments are not supported as owning comments"
                            )
                        }
                    },
                    None => {
                        unimplemented!(
                            "comments without metadata are not supported as owning comments"
                        )
                    }
                };
                if !is_owning {
                    break;
                }
                documents.push(leading_line_breaks(context, comment_start_offset, 1usize));
                let comment = context.comments.next().unwrap();
                documents.push(build_comment(&comment));
                documents.push(hardline());
                context.built_end = comment.location().end_offset();
            }
            None => break,
        }
    }
    if !documents.is_empty() {
        // remove last hardline and add break parent
        documents.pop();
        documents.push(break_parent());
        return Some(array(&documents));
    }
    None
}

/// Builds trailing comments for a given node.
/// ```ruby
/// foo # trailing comment
/// ```
pub fn trailing_comments(node: &Node, context: &mut BuildContext) -> Document {
    let mut documents = Vec::new();
    loop {
        match context.comments.peek() {
            Some(comment) => {
                let metadata = context
                    .comment_metadata
                    .get(&comment.location().start_offset());
                let is_trailing = match metadata {
                    Some(metadata) => {
                        if matches!(metadata.placement, CommentPlacement::EndOfLine) {
                            metadata.preceding_node_location.map_or(
                                false,
                                |preceding_node_location| {
                                    let node_start_offset = node.location().start_offset();
                                    let node_end_offset = node.location().end_offset();
                                    preceding_node_location.start_offset == node_start_offset
                                        && preceding_node_location.end_offset == node_end_offset
                                },
                            )
                        } else {
                            false
                        }
                    }
                    None => false,
                };
                if !is_trailing {
                    break;
                }
                let comment = context.comments.next().unwrap();
                let text = std::str::from_utf8(comment.text()).unwrap();
                documents.push(line_suffix(string(format!(" {}", text.trim_start()))));
                documents.push(break_parent());
                context.built_end = comment.location().end_offset();
            }
            None => break,
        }
    }
    array(&documents)
}

/// Builds the rest of the comments in the source code.
/// ```ruby
/// # rest comment
/// EOF
///```
pub fn rest_comments(context: &mut BuildContext) -> Document {
    let mut documents = Vec::new();
    loop {
        match context.comments.next() {
            Some(comment) => {
                documents.push(leading_line_breaks(
                    context,
                    comment.location().start_offset(),
                    1usize,
                ));
                documents.push(build_comment(&comment));
                documents.push(hardline());
                continue;
            }
            None => break,
        }
    }
    if !documents.is_empty() {
        // remove last hardline
        documents.pop();
    }
    array(&documents)
}

/// Builds a Document for a given comment.
/// If the comment is an embedded document comment (=begin ... =end),
/// it formats it as multiple lines with '#' prefixes.
fn build_comment(comment: &Comment) -> Document {
    let text = std::str::from_utf8(comment.text()).unwrap();
    match comment.type_() {
        CommentType::EmbDocComment => {
            let mut lines: Vec<&str> = text.lines().collect();
            if let Some(first) = lines.first() {
                if first.trim_start().starts_with("=begin") {
                    lines.remove(0);
                }
            }
            if let Some(last) = lines.last() {
                if last.trim_start().starts_with("=end") {
                    lines.pop();
                }
            }
            let mut documents = Vec::new();
            for (i, line) in lines.iter().enumerate() {
                if i > 0 {
                    documents.push(hardline());
                }
                documents.push(string(format!("# {}", line)));
            }
            array(&documents)
        }
        CommentType::InlineComment => string(text),
    }
}
