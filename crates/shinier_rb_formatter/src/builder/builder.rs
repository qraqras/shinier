use crate::doc::Doc;

pub trait Buildable<'sh> {
    fn build(&self) -> Doc;
}
