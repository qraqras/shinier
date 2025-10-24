use crate::keyword::Flag;

pub fn escape_slash_in_pattern(pattern: &[u8]) -> String {
    std::str::from_utf8(pattern).unwrap().replace("/", r"\/")
}

#[rustfmt::skip]
pub fn flags_string(
    is_ignore_case: bool,
    is_extended: bool,
    is_multi_line: bool,
    is_once: bool,
    is_euc_jp: bool,
    is_ascii_8bit: bool,
    is_windows_31j: bool,
    is_utf_8: bool,
) -> String {
    let mut flags = String::new();
    if is_ignore_case {flags.push(Flag::IgnoreCase.as_char());}
    if is_extended {flags.push(Flag::Extended.as_char());}
    if is_multi_line {flags.push(Flag::MultiLine.as_char());}
    if is_once {flags.push(Flag::Once.as_char());}
    if is_euc_jp {flags.push(Flag::EucJp.as_char());}
    if is_ascii_8bit {flags.push(Flag::Ascii8bit.as_char());}
    if is_windows_31j {flags.push(Flag::Windows31j.as_char());}
    if is_utf_8 {flags.push(Flag::Utf8.as_char());}
    flags
}
