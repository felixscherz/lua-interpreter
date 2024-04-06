use std::{
    fmt::Debug,
    fs::File,
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

    // +       -       *       /       %       ^       #
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Len,
    // &       ~       |       <<      >>      //
    BitAnd,
    BitXor,
    BitOr,
    ShiftL,
    ShiftR,
    Idiv,
    // ==       ~=     <=      >=      <       >        =
    Equal,
    NotEq,
    LesEq,
    GreEq,
    Less,
    Greater,
    Assign,
    // (       )       {       }       [       ]       ::
    ParL,
    ParR,
    CurlyL,
    CurlyR,
    SqurL,
    SqurR,
    DoubColon,
    // ;               :       ,       .       ..      ...
    SemiColon,
    Colon,
    Comma,
    Dot,
    Concat,
    Dots,

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
pub struct Lex<'a> {
    stream: &'a mut (dyn SeekRead + 'a),
}

impl<'a> Lex<'a> {
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
            'a' => {
                if let 'n' = self.read_char() {
                    if let 'd' = self.read_char() {
                        Token::And
                    } else {
                        self.seek(-1);
                        self.next()
                    }
                } else {
                    self.seek(-1);
                    self.next()
                }
            }
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
                let mut str = c.to_string();
                loop {
                    let c = self.read_char();
                    if c == ' ' {
                        break;
                    }
                    str.push(c);
                }
                Token::Name(str)
            }
            _ => panic!("Unexpected char in lexer"),
        }
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
    #[test]
    fn test_hello() {
        assert!(true)
    }
}
