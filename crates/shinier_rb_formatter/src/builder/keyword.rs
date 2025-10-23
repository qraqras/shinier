pub const ALIAS: &str = "alias";
pub const ALTERNATION: &str = "|";
pub const ARROW: &str = "->";
pub const BACK_QUOTE: &str = "`";
pub const BEGIN: &str = "begin";
pub const BRACES: (&str, &str) = ("{", "}");
pub const BRACKETS: (&str, &str) = ("[", "]");
pub const BREAK: &str = "break";

pub const CASE: &str = "case";
pub const CLASS: &str = "class";
pub const COLON: &str = ":";
pub const COMMA: &str = ",";

pub const DEF: &str = "def";
pub const DEFINED: &str = "defined?";
pub const DO: &str = "do";
pub const DOT_OPERATOR: &str = ".";
pub const DOUBLE_COLON: &str = "::";
pub const DOUBLE_DOT: &str = "..";
pub const DOUBLE_QUOTE: &str = "\"";

pub const ELSE: &str = "else";
pub const END: &str = "end";
pub const ENSURE: &str = "ensure";
pub const FALSE: &str = "false";
pub const FOR: &str = "for";

pub const HASH: &str = "#";

pub const IF: &str = "if";
pub const IMAGINARY: &str = "i";
pub const IN: &str = "in";
pub const INHERITES: &str = "<";
pub const IT: &str = "it";
pub const LOGICAL_AND: &str = "&&";
pub const LOGICAL_OR: &str = "||";
pub const PARENTHESES: (&str, &str) = ("(", ")");
pub const PIPE: &str = "|";
pub const PROC_AND: &str = "&";
pub const ROCKET: &str = "=>";
pub const SAFE_NAVIGATION_OPERATOR: &str = "&.";
pub const SEMI_COLON: &str = ";";
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
