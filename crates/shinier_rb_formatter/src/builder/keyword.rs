pub const ALIAS: &str = "alias";
pub const ALTERNATION: &str = "|";
pub const ARROW: &str = "->";
pub const ASTERISK: &str = "*";
pub const BACK_QUOTE: &str = "`";
pub const BEGIN: &str = "begin";
pub const BRACES: (&str, &str) = ("{", "}");
pub const BRACKETS: (&str, &str) = ("[", "]");
pub const BREAK: &str = "break";
pub const CARET: &str = "^";
pub const CASE: &str = "case";
pub const CLASS: &str = "class";
pub const COLON: &str = ":";
pub const COMMA: &str = ",";
pub const DOLLAR: &str = "$";
pub const DEF: &str = "def";
pub const DEFINED: &str = "defined?";
pub const DO: &str = "do";
pub const DOT_OPERATOR: &str = ".";
pub const DOUBLE_COLON: &str = "::";
pub const DOUBLE_DOT: &str = "..";
pub const DOUBLE_QUOTE: &str = "\"";
pub const ELSE: &str = "else";
pub const ENCODING: &str = "__ENCODING__";
pub const END: &str = "end";
pub const ENSURE: &str = "ensure";
pub const FALSE: &str = "false";
pub const FILE: &str = "__FILE__";
pub const FOR: &str = "for";
pub const HASH: &str = "#";
pub const IF: &str = "if";
pub const IMAGINARY: &str = "i";
pub const IN: &str = "in";
pub const INHERITES: &str = "<";
pub const IT: &str = "it";
pub const LINE: &str = "__LINE__";
pub const LOGICAL_AND: &str = "&&";
pub const LOGICAL_OR: &str = "||";
pub const MODULE: &str = "module";
pub const NEXT: &str = "next";
pub const NIL: &str = "nil";
pub const PARENTHESES: (&str, &str) = ("(", ")");
pub const PIPE: &str = "|";
pub const POST_EXECUTION: &str = "END";
pub const PRE_EXECUTION: &str = "BEGIN";
pub const PROC_AND: &str = "&";
pub const RATIONAL_SUFFIX: &str = "r";
pub const REDO: &str = "redo";
pub const RESCUE: &str = "rescue";
pub const RETRY: &str = "retry";
pub const RETURN: &str = "return";
pub const ROCKET: &str = "=>";
pub const SAFE_NAVIGATION_OPERATOR: &str = "&.";
pub const SELF: &str = "self";
pub const SEMI_COLON: &str = ";";
pub const SINGLETON: &str = "<<";
pub const SLASH: &str = "/";
pub const SPLAT: &str = "**";
pub const SUPER: &str = "super";
pub const THEN: &str = "then";
pub const TRIPLE_DOT: &str = "...";
pub const TRUE: &str = "true";
pub const WHEN: &str = "when";
pub const WRITE_OPERATOR: &str = "=";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    And,
    Or,
}

impl LogicalOperator {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::And => LOGICAL_AND,
            Self::Or => LOGICAL_OR,
        }
    }
}

pub enum Flag {
    IgnoreCase,
    Extended,
    MultiLine,
    Once,
    EucJp,
    Ascii8bit,
    Windows31j,
    Utf8,
}

impl Flag {
    pub const fn as_char(&self) -> char {
        match self {
            Self::IgnoreCase => 'i',
            Self::Extended => 'x',
            Self::MultiLine => 'm',
            Self::Once => 'o',
            Self::EucJp => 'e',
            Self::Ascii8bit => 'n',
            Self::Windows31j => 's',
            Self::Utf8 => 'u',
        }
    }
}

pub enum NumberBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl NumberBase {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Binary => "0b",
            Self::Octal => "0o",
            Self::Decimal => "", // 0d
            Self::Hexadecimal => "0x",
        }
    }
}

pub enum ShareableConstantComment {
    Literal,
    ExperimentalEverything,
    ExperimentalCopy,
}

impl ShareableConstantComment {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Literal => "# shareable_constant_value: literal",
            Self::ExperimentalEverything => "# shareable_constant_value: experimental_everything",
            Self::ExperimentalCopy => "# shareable_constant_value: experimental_copy",
        }
    }
}
