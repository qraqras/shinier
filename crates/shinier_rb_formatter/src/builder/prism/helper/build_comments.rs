use crate::BuildContext;
use crate::builder::builder::{array, break_parent, hardline, line_suffix, string};
use crate::builder::prism::VisitAll;
use crate::builder::prism::blank_lines;
use crate::builder::prism::helper::build_blank_lines::LineBreakIndex;
use crate::document::Document;
use ruby_prism::Comment;
use ruby_prism::CommentType;
use ruby_prism::Node;

/// Metadata about comments used for advanced placement logic.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
impl<'sh> VisitAll<'sh> for NodeLocationCollector<'sh> {
    fn node_enter(&mut self, node: &Node<'sh>) {
        self.node_locations.push(NodeLocation {
            start_offset: node.location().start_offset(),
            end_offset: node.location().end_offset(),
        });
    }
}

/// Sorted node locations by both start_offset and end_offset.
pub struct SortedNodeLocations {
    pub by_start: Vec<NodeLocation>,
    pub by_end: Vec<NodeLocation>,
}

/// Collect node locations sorted by both start_offset and end_offset.
pub fn collect_sorted_node_locations<'sh>(node: &Node<'sh>) -> SortedNodeLocations {
    let mut node_locations = Vec::new();
    let mut collector = NodeLocationCollector {
        node_locations: &mut node_locations,
    };
    collector.visit(node);
    let mut by_start = node_locations.clone();
    let mut by_end = node_locations.clone();
    by_start.sort_unstable_by_key(|loc| loc.start_offset);
    by_end.sort_unstable_by_key(|loc| loc.end_offset);
    SortedNodeLocations { by_start, by_end }
}

/// Decorates a comment with metadata for placement decisions.
pub fn decorate_comment<'sh>(
    comment: &'sh Comment<'sh>,
    sorted_node_locations: &'sh SortedNodeLocations,
    line_break_index: &LineBreakIndex,
) -> CommentMetadata {
    let comment_start_offset = comment.location().start_offset();
    let comment_end_offset = comment.location().end_offset();
    // Find preceding node - binary search for nodes ending before comment
    let preceding_node_location = find_preceding_node(
        &sorted_node_locations,
        comment_start_offset,
        line_break_index,
    );
    // Find following node - binary search for nodes starting after comment
    let following_node_location = find_following_node(&sorted_node_locations, comment_end_offset);
    // Find enclosing node - needs to scan all candidates
    let enclosing_node_location = find_enclosing_node(
        &sorted_node_locations,
        comment_start_offset,
        comment_end_offset,
    );
    let placement = determine_placement(
        comment,
        preceding_node_location.as_ref(),
        following_node_location.as_ref(),
        line_break_index,
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
fn find_preceding_node(
    sorted_node_locations: &SortedNodeLocations,
    comment_start: usize,
    line_break_index: &LineBreakIndex,
) -> Option<NodeLocation> {
    let sorted = &sorted_node_locations.by_end;
    let idx = sorted.partition_point(|node| node.end_offset < comment_start);
    if idx == 0 {
        return None;
    }
    let mut result: Option<NodeLocation> = None;
    let mut found_single_line_node = false;
    for node in sorted[..idx].iter().rev() {
        // node end and comment start are on different lines
        if line_break_index.has_line_break_in_range(node.end_offset, comment_start) {
            break;
        }
        if result.is_none_or(|r| r.end_offset <= node.end_offset) {
            let mut should_update = false;
            // single-line node
            if line_break_index.has_line_break_in_range(node.start_offset, node.end_offset) {
                should_update = match result {
                    Some(prev) => node.start_offset < prev.start_offset,
                    None => true,
                };
                found_single_line_node = true;
            // multi-line node
            } else {
                if found_single_line_node {
                    break;
                }
                should_update = match result {
                    Some(prev) => prev.start_offset < node.start_offset,
                    None => true,
                };
            }
            if should_update {
                result = Some(*node);
            }
            continue;
        }
        break;
    }
    result
}

/// Finds the following node (comment_end <= node_start).
fn find_following_node(
    sorted_node_locations: &SortedNodeLocations,
    comment_end: usize,
) -> Option<NodeLocation> {
    let sorted = &sorted_node_locations.by_start;
    let idx = sorted.partition_point(|node| node.start_offset < comment_end);
    if idx >= sorted.len() {
        return None;
    }
    let start_idx = idx;
    let mut result = sorted[start_idx];
    for node in sorted[start_idx + 1..].iter() {
        if node.start_offset == result.start_offset {
            if node.end_offset < result.end_offset {
                result = *node;
            }
            continue;
        }
        break;
    }
    Some(result)
}

/// Finds the smallest enclosing node (node_start <= comment_start <= comment_end <= node_end).
fn find_enclosing_node(
    sorted_node_locations: &SortedNodeLocations,
    comment_start: usize,
    comment_end: usize,
) -> Option<NodeLocation> {
    let sorted = &sorted_node_locations.by_start;
    let idx = sorted.partition_point(|node| node.start_offset <= comment_start);
    let mut result: Option<NodeLocation> = None;
    for node in sorted[..idx].iter().rev() {
        if node.end_offset < comment_end {
            break;
        }
        match result {
            Some(prev) => {
                let prev_size = prev.end_offset - prev.start_offset;
                let curr_size = node.end_offset - node.start_offset;
                if curr_size < prev_size {
                    result = Some(*node);
                }
            }
            None => result = Some(*node),
        }
    }
    result
}

/// Determines the placement of a comment based on surrounding nodes and source code.
pub fn determine_placement(
    comment: &Comment,
    preceding_node_location: Option<&NodeLocation>,
    following_node_location: Option<&NodeLocation>,
    line_break_index: &LineBreakIndex,
) -> CommentPlacement {
    fn has_newline_in_range(
        line_break_index: &LineBreakIndex,
        start_offset: usize,
        end_offset: usize,
    ) -> bool {
        line_break_index.has_line_break_in_range(start_offset, end_offset)
    }
    let comment_start_offset = comment.location().start_offset();
    let comment_end_offset = comment.location().end_offset();
    let has_newline_before = preceding_node_location.map_or(true, |loc| {
        has_newline_in_range(line_break_index, loc.end_offset, comment_start_offset)
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
                has_newline_in_range(line_break_index, comment_end_offset - 1, loc.start_offset)
            }
            CommentType::InlineComment => {
                has_newline_in_range(line_break_index, comment_end_offset, loc.start_offset)
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
pub fn leading_comments(node: &Node, context: &mut BuildContext) -> Option<Document> {
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
                if let Some(blank_lines) =
                    blank_lines(context, context.built_end, comment_start_offset, 1usize)
                {
                    documents.push(blank_lines);
                }
                let comment = context.comments.next().unwrap();
                documents.push(build_comment(&comment));
                documents.push(hardline());
                context.built_end = comment.location().end_offset();
            }
            None => break,
        }
    }
    match documents.is_empty() {
        true => None,
        false => Some(array(&documents)),
    }
}

/// Builds owning comments for a given node.
/// ```ruby
/// if foo then
///   # owning comment
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
                if !documents.is_empty() {
                    documents.push(hardline());
                }
                let gap_start_offset = context.built_end.max(node.location().start_offset());
                let gap_end_offset = comment_start_offset;
                if let Some(blank_lines) =
                    blank_lines(context, gap_start_offset, gap_end_offset, 1usize)
                {
                    documents.push(blank_lines);
                }
                let comment = context.comments.next().unwrap();
                documents.push(build_comment(&comment));
                context.built_end = comment.location().end_offset();
            }
            None => break,
        }
    }
    match documents.is_empty() {
        true => None,
        false => {
            documents.push(break_parent()); // ensure proper breaking behavior
            Some(array(&documents))
        }
    }
}

/// Builds owning comments for a given node with optional documents before and after.
/// ```ruby
/// if foo then
///   {before}# owning comment
///   # owning comment{after}
/// end
/// ```
pub fn owning_comments_with(
    node: &Node,
    context: &mut BuildContext,
    before: Option<Document>,
    after: Option<Document>,
) -> Option<Document> {
    match owning_comments(node, context) {
        Some(comments) => {
            let mut documents = Vec::new();
            if let Some(before) = before {
                documents.push(before);
            }
            documents.push(comments);
            if let Some(after) = after {
                documents.push(after);
            }
            Some(array(&documents))
        }
        None => None,
    }
}

/// Builds trailing comments for a given node.
/// ```ruby
/// foo # trailing comment
/// ```
pub fn trailing_comments(node: &Node, context: &mut BuildContext) -> Option<Document> {
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
    match documents.is_empty() {
        true => None,
        false => Some(array(&documents)),
    }
}

/// Builds the rest of the comments in the source code.
/// ```ruby
/// # rest comment
/// EOF
///```
pub fn dangling_comments(context: &mut BuildContext) -> Option<Document> {
    let mut documents = Vec::new();
    loop {
        match context.comments.next() {
            Some(comment) => {
                if let Some(blank_lines) = blank_lines(
                    context,
                    context.built_end,
                    comment.location().start_offset(),
                    1usize,
                ) {
                    documents.push(blank_lines);
                }
                documents.push(build_comment(&comment));
                documents.push(hardline());
                continue;
            }
            None => break,
        }
    }
    match documents.is_empty() {
        true => None,
        false => {
            documents.pop(); // remove last hardline
            return Some(array(&documents));
        }
    }
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

pub fn keyword_trailing_comments(
    keyword_end_offset: usize,
    context: &mut BuildContext,
) -> Option<Document> {
    fn has_newline_in_range(
        line_break_index: &LineBreakIndex,
        start_offset: usize,
        end_offset: usize,
    ) -> bool {
        line_break_index.has_line_break_in_range(start_offset, end_offset)
    }
    let mut documents = Vec::new();
    loop {
        match context.comments.peek() {
            Some(comment) => {
                let comment_start_offset = comment.location().start_offset();
                if comment_start_offset < keyword_end_offset {
                    break;
                }
                if has_newline_in_range(
                    &context.line_break_index,
                    keyword_end_offset,
                    comment_start_offset,
                ) {
                    break;
                }
                let comment = context.comments.next().unwrap();
                documents.push(line_suffix(build_comment(&comment)));
                documents.push(break_parent());
                context.built_end = comment.location().end_offset();
            }
            None => break,
        }
    }
    match documents.is_empty() {
        true => None,
        false => Some(array(&documents)),
    }
}
