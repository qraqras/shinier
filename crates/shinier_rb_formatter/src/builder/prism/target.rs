use ruby_prism::Location;
use ruby_prism::Node;

pub enum Target<'sh> {
    Location(&'sh Location<'sh>),
    Node(&'sh Node<'sh>),
}
impl<'sh> Target<'sh> {
    pub fn start_offset(&self) -> usize {
        match self {
            Target::Location(l) => l.start_offset(),
            Target::Node(n) => n.location().start_offset(),
        }
    }
    pub fn end_offset(&self) -> usize {
        match self {
            Target::Location(l) => l.end_offset(),
            Target::Node(n) => n.location().end_offset(),
        }
    }
}
impl<'sh> From<&'sh Node<'sh>> for Target<'sh> {
    fn from(node: &'sh Node<'sh>) -> Self {
        Target::Node(node)
    }
}
impl<'sh> From<&'sh Location<'sh>> for Target<'sh> {
    fn from(loc: &'sh Location<'sh>) -> Self {
        Target::Location(loc)
    }
}
