/// Describes how a sequence of token trees is delimited.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Delimiter {
    /// `( ... )`
    Parenthesis,
    /// `{ ... }`
    Brace,
    /// `[ ... ]`
    Bracket,
}

/// Types of literals.
#[derive(Clone, Copy, PartialEq, Encodable, Decodable, Debug)]
pub enum LiteralKind {
    Boolean,
    Char,
    Integer,
    Real,
    String,
}

/// A literal token.
/// Literals are things like numbers, string, and characters as described in the program through
/// syntax. An example of a number literal is `123`, and an example of a string literal is `"abc"`.
#[derive(Clone, Copy, PartialEq, Encodable, Decodable, Debug)]
pub struct Literal {
    pub kind: LiteralKind,
    pub symbol: Symbol,
    pub suffix: Option<Symbol>,
}

#[derive(Clone, PartialEq, Encodable, Decodable, Debug)]
pub enum TokenKind {
    BinaryOper(),

    // Math Operators
    Add,
    Divide,
    Modulus,
    Multiply,
    Subtract,
    // Comparison operators
    Equals,
    LessEqual,
    LessThan,
    GreaterEqual,
    GreaterThan,
    ValueEquals,
    // Boolean operators
    And,
    AndThen,
    Not,
    Or,
    OrElse,
    LeftShift,
    RightShift,

    /// An opening delimiter (e.g., `{`)
    OpenDelim(Delimiter),
    /// A closing delimiter (e.g., `}`)
    CloseDelim(Delimiter),
    /// Literals
    Literals(Literal),

    /// Identifier token.
    ///
    Ident(Symbol),
}
