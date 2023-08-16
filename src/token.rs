// Struct to store Token data
struct Token {
    token_type: Option<TokenType>,
    value: Option<i32>,
}

impl Token {
    pub fn new(token_type: Option<TokenType>, value: Option<i32>) -> Self {
        Token {
            token_type,
            value,
        }
    }

    pub fn set_type(self, token_type: Option<TokenType>) {
        self.token_type = token_type
    }

    pub fn set_value(self, value: Option<i32>) {
        self.value = value
    }
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
