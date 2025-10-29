use crate::BuildContext;
use crate::Build;
use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::helper::regex::{escape_slash_in_pattern, flags_string};
use crate::keyword::SLASH;
use ruby_prism::RegularExpressionNode;

impl<'sh> Build for Option<&RegularExpressionNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&RegularExpressionNode>, _context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let is_ignore_case = node.is_ignore_case();
    let is_extended = node.is_extended();
    let is_multi_line = node.is_multi_line();
    let is_once = node.is_once();
    let is_euc_jp = node.is_euc_jp();
    let is_ascii_8bit = node.is_ascii_8bit();
    let is_windows_31j = node.is_windows_31j();
    let is_utf_8 = node.is_utf_8();
    let unescaped = node.unescaped();
    array(&[
        string(SLASH),
        string(escape_slash_in_pattern(unescaped)),
        string(SLASH),
        string(flags_string(
            is_ignore_case,
            is_extended,
            is_multi_line,
            is_once,
            is_euc_jp,
            is_ascii_8bit,
            is_windows_31j,
            is_utf_8,
        )),
    ])
}
