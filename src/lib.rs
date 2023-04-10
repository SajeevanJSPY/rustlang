mod cursor;

struct TokenKind {
    kind: Token,
}

/// Every Tokens Covered By Rust
enum Token {
    /// Ident - keyword, variable name
    Ident,
    /// 's'
    Char {
        terminated: bool,
    },
    /// "str_example"
    Str {
        terminated: bool,
    },
    /// Whitespace
    WhiteSpace,
    /// // Line Comment
    LineComment {
        doc_style: Option<DocStyle>,
    },
    /// /* */
    BlockComment {
        terminated: bool,
    },
    /// (
    OpenParan,
    /// )
    CloseParan,
    /// {
    OpenBrace,
    /// }
    CloseBrace,
    /// [
    OpenBracket,
    /// ]
    CloseBracket,
    /// >
    Gt,
    /// <
    Lt,
    /// =
    Eq,
    /// &
    And,
    /// |
    Or,
    /// +
    Plus,
    /// -
    Minus,
    /// #
    Pound,
    /// @
    At,
    /// .
    Dot,
    /// /
    Slash,
    /// ,
    Comma,
    /// *
    Star,
    /// !
    Not,
    /// _
    Underscore,
    /// :
    Colon,
    /// ;
    SemiColon,
    /// ?
    Question,
    /// $
    Dollar,
    /// ~
    Tilde,
    /// ^
    Caret,
    /// %
    Percent,
    Unknown,
    Eof,
}

enum DocStyle {
    Outer,
    Inner,
}

enum LiteralKind {
    Int { base: Base },
    Float { base: Base },
    Char { terminated: bool },
    Str { terminated: bool },
}

enum Base {
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    HexaDecimal = 16,
}
