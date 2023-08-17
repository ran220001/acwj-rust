use token::{Token, TokenType};

struct Scanner {
    filename: &str,
    file: File, // std::fs::File
    putback: Option<char>,
    line_number: i32,
}

impl Scanner {
    pub fn new(filename: &str) -> Self {
        Scanner {
            filename,
            file = File::open(filename),
            putback = None,
            line_number = 1,
        }
    }

    fn next(self) -> Option<char> {
        let mut c: char.default();

        // If a character exists in the putback buffer,
        // use it instead of reading from the input file
        if let Some(p) = self.putback {
            c = Some(p);
            self.putback = None;
        } else {
            c = match read_char(self.file) { // todo placeholder!
                Err(reason) => panic!("{:?}", reason),
                Ok(_) => _,
            }
            if c == Some('\n') {
                // Increment line number with every new line
                self.line_number += 1;
            }
        }

        c
    }

    fn putback(self, c: char) {
        self.putback = Some(c);
    }

    fn skip(self) -> Option<char> {
        let mut c = self.next();
        while let Some(thing) = c {
            if thing.is_whitespace() {
                c = self.next();
            } else {
                break;
            }
        }

        c
    }

    fn scan(self, t: Token) -> bool {
        let mut c = skip();
        
        if let Some(token) = c {
            match token {
                '+' => t.set_type(Some(TokenType::Plus)),
                '-' => t.set_type(Some(TokenType::Minus)),
                '*' => t.set_type(Some(TokenType::Star)),
                '/' => t.set_type(Some(TokenType::Slash)),
                _ => {
                    if token.is_ascii_digit() {
                        t.set_value(Some(scan_int(c)));
                        t.set_type(Some(TokenType::IntegerLiteral));
                    } else {
                        panic!("Unrecognised character {c} on line {self.line_number}");
                    }
                }

            True
        } else {
            False
        }
    }
}
