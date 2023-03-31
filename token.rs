#[derive(Debug, Clone, PartialEq,Copy)]
pub enum Kind<'a> {
    Eof,
    Error,
    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Equals,
    EqualsEquals,

    Identifier(&'a str),
    Int(&'a str),
    Float(&'a str),
    String(&'a str),

    And,
    Or,
    LBrac,
    RBrac,
    LAngleBrac,
    RAngleBrac,
    LSquareBrac,
    RSquareBrac,
    LCurlyBrac,
    RCurlyBrac,
    Ques,
    Bang,
    Star,
    Slash,
    Amp,
    Caret,
    Percent,


    // Keywords
    Else,
    False,
    True,
    Return,
    If,
    While,
    For,
    Var,
    Class,
    This,
    Fun,
    Import,
    Null,
}

impl<'a> Kind<'a> {
    pub fn to_str(self) -> &'a str {
        match self {
            Kind::Plus => "+",
            Kind::PlusPlus => "++",
            Kind::Eof => "EOF",
            _ => ""
        }
    }
}

impl<'a> From<&'a str> for Kind<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        match value {
            "if" => Self::If,
            "true" => Self::True,
            "false" => Self::False,
            "return" => Self::Return,
            "fun" => Self::Fun,
            "import" => Self::Import,
            "class" => Self::Class,
            "this" => Self::This,
            "null" => Self::Null,
            "var" => Self::Var,
            "while" => Self::While,
            "else" => Self::Else,
            "for" => Self::For,
            _ => Self::Identifier(value)
        }
    }
}
