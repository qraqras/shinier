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
struct Target<'sh> {
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

fn is_trailing(comment: &Comment) -> bool {
    // !location.start_line_slice.strip.empty?
    // TODO: implement
    // コメントから行頭までをstripして空ならfalse
    false
}

fn comment_targets<'sh>(node: &'sh Node<'sh>) -> Vec<Target<'sh>> {
    let targets = match node {
        // TODO: StatementsNodeの場合はbodyの各ノードをターゲットに追加すること
        // TODO: ノード事にターゲットを追加すること
        _ => Vec::new(),
    };
    targets
}
