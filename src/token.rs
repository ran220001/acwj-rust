// Struct to store Token data
#[derive(Default)]
struct Token {
    type: Option<TokenType>,
    value: Option<i32>,
}


enum TokenType {
    // Operators
    Plus,
    Minus,
    Star,
    Slash,

    // Literals
    IntegerLiteral,
}
