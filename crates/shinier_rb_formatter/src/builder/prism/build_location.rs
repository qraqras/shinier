use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::helper::build_comments::*;
use ruby_prism::*;
use std::io::Read;

pub fn build_location(location: &Location, context: &mut BuildContext) -> Document {
    let leading_comments = leading_comments_l(location, context);
    let trailing_comments = trailing_comments_l(location, context);
    let mut buf = String::new();
    let _ = location.as_slice().read_to_string(&mut buf);
    array_opt(&[leading_comments, Some(string(buf)), trailing_comments])
}
