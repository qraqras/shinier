use ruby_prism::*;
use std::collections::HashMap;

struct CommentVisitor<'a> {
    pub comments: Comments<'a>,
    pub index: usize,
    pub comment_location_map: HashMap<usize, Comment<'a>>,
}
impl<'a> CommentVisitor<'a> {
    pub fn new(comments: Comments<'a>) -> Self {
        Self {
            comments,
            index: 0,
            comment_location_map: HashMap::new(),
        }
    }
}
impl<'a> Visit<'a> for CommentVisitor<'a> {
    fn visit_branch_node_enter(&mut self, node: Node<'a>) {
        if let Some(comment) = self.comments.nth(self.index) {
            self.index += 1;
            self.comment_location_map
                .insert(node.location().start_offset(), comment);
        }
    }
    fn visit_leaf_node_enter(&mut self, node: Node<'a>) {
        if let Some(comment) = self.comments.nth(self.index) {
            self.index += 1;
            self.comment_location_map
                .insert(node.location().start_offset(), comment);
        }
    }
}

pub fn attach_comments<'a>(parse_result: &'a ParseResult<'a>) -> HashMap<usize, Comment<'a>> {
    let mut visitor = CommentVisitor::new(parse_result.comments());
    visitor.visit(&parse_result.node());
    visitor.comment_location_map
}
