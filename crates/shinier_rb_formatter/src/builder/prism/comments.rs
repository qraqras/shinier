// https://github.com/ruby/prism/blob/main/lib/prism/parse_result/comments.rb
use ruby_prism::*;

/// trait for attaching comments to targets
trait Attach<'sh> {
    fn start_offset(&self) -> usize;
    fn end_offset(&self) -> usize;
    fn encloses(&self, comment: &Comment<'sh>) -> bool;
    fn leading_comment(&self, comment: &Comment<'sh>);
    fn trailing_comment(&self, comment: &Comment<'sh>);
}

/// target node for attaching comments
struct NodeTarget<'sh> {
    node: &'sh Node<'sh>,
}
impl<'sh> NodeTarget<'sh> {
    fn new(node: &'sh Node<'sh>) -> Self {
        NodeTarget { node }
    }
}
impl<'sh> Attach<'sh> for NodeTarget<'sh> {
    fn start_offset(&self) -> usize {
        self.node.location().start_offset()
    }
    fn end_offset(&self) -> usize {
        self.node.location().end_offset()
    }
    fn encloses(&self, comment: &Comment<'sh>) -> bool {
        self.start_offset() <= comment.location().start_offset()
            && comment.location().end_offset() <= self.end_offset()
    }
    fn leading_comment(&self, _comment: &Comment<'sh>) {
        // TODO: implement
    }
    fn trailing_comment(&self, _comment: &Comment<'sh>) {
        // TODO: implement
    }
}

/// target location for attaching comments
struct LocationTarget<'sh> {
    location: &'sh Location<'sh>,
}
impl<'sh> LocationTarget<'sh> {
    fn new(location: &'sh Location<'sh>) -> Self {
        LocationTarget { location }
    }
}
impl<'sh> Attach<'sh> for LocationTarget<'sh> {
    fn start_offset(&self) -> usize {
        self.location.start_offset()
    }
    fn end_offset(&self) -> usize {
        self.location.end_offset()
    }
    fn encloses(&self, _comment: &Comment<'sh>) -> bool {
        false
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
    for comment in parse_result.comments() {
        let (preceding, enclosing, following) = nearest_targets(node, &comment);
        if is_trailing(&comment) {
            if let Some(preceding) = preceding {
                preceding.trailing_comment(&comment);
            } else {
                if let Some(following) = following {
                    following.leading_comment(&comment);
                } else if let Some(enclosing) = enclosing {
                    enclosing.leading_comment(&comment);
                } else {
                    NodeTarget::new(&parse_result.node()).leading_comment(&comment);
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
                    NodeTarget::new(&parse_result.node()).leading_comment(&comment);
                }
            }
        }
    }
}

fn nearest_targets<'sh>(
    node: &'sh Node<'sh>,
    comment: &'sh Comment<'sh>,
) -> (
    Option<NodeTarget<'sh>>,
    Option<NodeTarget<'sh>>,
    Option<NodeTarget<'sh>>,
) {
    let comment_start = comment.location().start_offset();
    let comment_end = comment.location().end_offset();

    let mut targets: Vec<Box<dyn Attach<'sh>>> = Vec::new();
    for value in comment_targets(node) {
        match value {
            CommentTarget::Node(n) => {
                targets.push(Box::new(NodeTarget::new(n)));
            }
            CommentTarget::Location(l) => {
                targets.push(Box::new(LocationTarget::new(l)));
            }
        }
    }

    (None, None, None)
}

fn is_trailing(comment: &Comment) -> bool {
    // !location.start_line_slice.strip.empty?
    // TODO: implement
    // コメントから行頭までをstripして空ならfalse
    false
}

enum CommentTarget<'sh> {
    Node(&'sh Node<'sh>),
    Location(&'sh Location<'sh>),
}
fn comment_targets<'sh>(node: &'sh Node<'sh>) -> Vec<CommentTarget<'sh>> {
    let targets = match node {
        // TODO: StatementsNodeの場合はbodyの各ノードをターゲットに追加すること
        // TODO: ノード事にターゲットを追加すること
        _ => Vec::new(),
    };
    targets
}
