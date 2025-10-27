use crate::BuildPrismNode;
use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::{Comments, Location};
use std::collections::HashMap;

impl<'a> BuildPrismNode for Location<'_> {
    fn _build(&self, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
        match std::str::from_utf8(self.as_slice()) {
            Ok(s) => string(s),
            Err(_) => string("<invalid utf8>"),
        }
    }
}
