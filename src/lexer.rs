use std::{
    fmt::Debug,
    io::{Read, Seek, SeekFrom},
};

#[derive(Debug, PartialEq)]
pub enum Token {
    // keywords
    And,
    Break,
    Do,
    Else,
    Elseif,
    End,
    False,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,

    // symbols
    Add,       // +
    Sub,       // -
    Mul,       // *
    Div,       // /
    Mod,       // %
    Pow,       // ^
    Len,       // #
    BitAnd,    // &
    BitXor,    // ~
    BitOr,     // |
    ShiftL,    // <<
    ShiftR,    // >>
    Idiv,      // //
    Equal,     // ==
    NotEq,     // ~=
    LesEq,     // <=
    GreEq,     // >=
    Less,      // <
    Greater,   // >
    Assign,    // =
    ParL,      // (
    ParR,      // )
    CurlyL,    // {
    CurlyR,    // }
    SqurL,     // [
    SqurR,     // ]
    DoubColon, // ::
    SemiColon, // ;
    Colon,     // :
    Comma,     // ,
    Dot,       // .
    Concat,    // ..
    Dots,      // ...

    // constant values
    Integer(i64),
    Float(f64),
    String(String),

    // name of variables or table keys
    Name(String),

    // end
    Eos,
}

pub trait SeekRead: Seek + Read + Debug {}
impl<T> SeekRead for T where T: Seek + Read + Debug {}

#[derive(Debug)]
pub struct Lexer<'a> {
    stream: &'a mut (dyn SeekRead + 'a),
}

impl<'a> Lexer<'a> {
    pub fn new(stream: &'a mut (dyn SeekRead + 'a)) -> Self {
        Self { stream }
    }

    fn seek(&mut self, n: i64) {
        self.stream.seek(SeekFrom::Current(n)).unwrap();
    }

    pub fn next(&mut self) -> Token {
        let c = self.read_char();
        match c {
            '\0' => Token::Eos,
            ' ' | '\n' | '\t' | '\r' => self.next(),
            '"' => {
                let mut str = String::new();
                loop {
                    let c = self.read_char();
                    if c == '"' {
                        break;
                    }
                    str.push(c);
                }
                Token::String(str)
            }
            'A'..='Z' | 'a'..='z' | '_' => {
                self.seek(-1);
                self.read_word()
            }
            '0'..='9' => {
                let mut str = c.to_string();
                let mut is_float = false;
                loop {
                    let c = self.read_char();
                    match c {
                        '0'..='9' => str.push(c),
                        '.' => {
                            is_float = true;
                            str.push(c)
                        }
                        _ => {
                            self.seek(-1);
                            break;
                        }
                    }
                }
                if is_float {
                    Token::Float(str.parse().unwrap())
                } else {
                    Token::Integer(str.parse().unwrap())
                }
            }
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '/' => Token::Div,
            '%' => Token::Mod,
            '^' => Token::Pow,
            '#' => Token::Len,
            '&' => Token::BitAnd,
            '=' => {
                let c = self.read_char();
                match c {
                    '=' => Token::Equal,
                    _ => {
                        self.seek(-1);
                        Token::Assign
                    }
                }
            }
            '~' => {
                let c = self.read_char();
                match c {
                    '=' => Token::NotEq,
                    _ => {
                        self.seek(-1);
                        Token::BitXor
                    }
                }
            }
            '|' => Token::BitOr,
            '(' => Token::ParL,
            ')' => Token::ParR,
            '{' => Token::CurlyL,
            '}' => Token::CurlyR,
            '[' => Token::SqurL,
            ']' => Token::SqurR,
            ':' => {
                let c = self.read_char();
                match c {
                    ':' => Token::DoubColon,
                    _ => {
                        self.seek(-1);
                        Token::Colon
                    }
                }
            }
            ';' => Token::SemiColon,
            ',' => Token::Comma,
            '.' => {
                let c = self.read_char();
                match c {
                    '.' => {
                        let c = self.read_char();
                        match c {
                            '.' => Token::Dots,
                            _ => {
                                self.seek(-1);
                                Token::Concat
                            }
                        }
                    }
                    _ => {
                        self.seek(-1);
                        Token::Dot
                    }
                }
            }
            c if self.match_pattern(c, "<<") => Token::ShiftL,
            '<' => Token::Less,
            c if self.match_pattern(c, ">>") => Token::ShiftR,
            '>' => Token::Greater,
            c if self.match_pattern(c, "//") => Token::Idiv,
            c if self.match_pattern(c, "<=") => Token::LesEq,
            c if self.match_pattern(c, ">=") => Token::GreEq,
            _ => panic!("Unexpected char in lexer"),
        }
    }

    fn read_word(&mut self) -> Token {
        let mut word = String::new();
        loop {
            let c = self.read_char();
            if c == ' ' {
                break;
            }
            word.push(c);
        }
        match word.as_str() {
            "and" => Token::And,
            "break" => Token::Break,
            "do" => Token::Do,
            "else" => Token::Else,
            "elseif" => Token::Elseif,
            "end" => Token::End,
            "false" => Token::False,
            "for" => Token::For,
            "function" => Token::Function,
            "goto" => Token::Goto,
            "if" => Token::If,
            "in" => Token::In,
            "local" => Token::Local,
            "nil" => Token::Nil,
            "not" => Token::Not,
            "or" => Token::Or,
            "repeat" => Token::Repeat,
            "return" => Token::Return,
            "then" => Token::Then,
            "true" => Token::True,
            "until" => Token::Until,
            "while" => Token::While,
            _ => Token::Name(word),
        }
    }

    fn match_pattern(&mut self, start: char, pattern: &str) -> bool {
        let mut n = 0;
        let mut next_char = start;
        for c in pattern.chars() {
            if next_char != c {
                self.seek(-n);
                return false;
            }
            next_char = self.read_char();
            n += 1;
        }
        true
    }

    fn read_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        if self.stream.read(&mut buf).unwrap() == 1 {
            buf[0] as char
        } else {
            '\0' // null-byte signifies end of file
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_print_hello_world() {
        let code = "print \"hello world!\"".to_string();
        let mut cursor = Cursor::new(code);
        let mut lexer = Lexer::new(&mut cursor);
        assert_eq!(lexer.next(), Token::Name("print".to_string()));
        assert_eq!(lexer.next(), Token::String("hello world!".to_string()));
        assert_eq!(lexer.next(), Token::Eos);
    }

    #[test]
    fn test_parse_strings() {
        let code = "\"<< >>\"".to_string();
        let mut cursor = Cursor::new(code);
        let mut lexer = Lexer::new(&mut cursor);
        assert_eq!(lexer.next(), Token::String("<< >>".to_string()));
    }

    #[test]
    fn test_parse_integer() {
        let code = "local a = 0".to_string();
        let mut cursor = Cursor::new(code);
        let mut lexer = Lexer::new(&mut cursor);
        assert_eq!(lexer.next(), Token::Local);
        assert_eq!(lexer.next(), Token::Name("a".to_string()));
        assert_eq!(lexer.next(), Token::Assign);
        assert_eq!(lexer.next(), Token::Integer(0));
    }

    #[test]
    fn test_parse_float() {
        let code = "local a = 0.5".to_string();
        let mut cursor = Cursor::new(code);
        let mut lexer = Lexer::new(&mut cursor);
        assert_eq!(lexer.next(), Token::Local);
        assert_eq!(lexer.next(), Token::Name("a".to_string()));
        assert_eq!(lexer.next(), Token::Assign);
        assert_eq!(lexer.next(), Token::Float(0.5));
    }

    #[test]
    fn test_parse_dots() {
        let code = ". .. ...".to_string();
        let mut cursor = Cursor::new(code);
        let mut lexer = Lexer::new(&mut cursor);
        assert_eq!(lexer.next(), Token::Dot);
        assert_eq!(lexer.next(), Token::Concat);
        assert_eq!(lexer.next(), Token::Dots);
    }

    #[test]
    fn test_parse_shifts() {
        let code = "<< >> < >".to_string();
        let mut cursor = Cursor::new(code);
        let mut lexer = Lexer::new(&mut cursor);
        assert_eq!(lexer.next(), Token::ShiftL);
        assert_eq!(lexer.next(), Token::ShiftR);
        assert_eq!(lexer.next(), Token::Less);
        assert_eq!(lexer.next(), Token::Greater);
    }

    #[test]
    fn test_parse_addition() {
        let code = "5+5".to_string();
        let mut cursor = Cursor::new(code);
        let mut lexer = Lexer::new(&mut cursor);
        assert_eq!(lexer.next(), Token::Integer(5));
        assert_eq!(lexer.next(), Token::Add);
        assert_eq!(lexer.next(), Token::Integer(5));
    }
}
