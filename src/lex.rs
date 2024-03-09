use std::{fs::File, io::Read};

#[derive(Debug)]
pub enum Token {
    Name(String),
    String(String),
    Eos,
}

#[derive(Debug)]
pub struct Lex {
    input: File,
}

impl Lex {
    pub fn new(input: File) -> Self {
        Self { input }
    }

    pub fn next(&mut self) -> Token {
        self.tokenize()
    }

    fn tokenize(&mut self) -> Token {
        let c = self.read_char();
        match c {
            '\0' => Token::Eos,
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
            ' ' | '\n' | '\t' => self.tokenize(),
            c => {
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
        }
    }

    fn read_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        if self.input.read(&mut buf).unwrap() == 1 {
            buf[0] as char
        } else {
            '\0' // null-byte signifies end of file
        }
    }
}
