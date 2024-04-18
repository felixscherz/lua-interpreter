use crate::bytecode::ByteCode;
use crate::{
    lexer::{Lexer, Token},
    value::Value,
};
use std::fs::File;

#[derive(Debug)]
pub struct ParseProto {
    pub constants: Vec<Value>,
    pub byte_codes: Vec<ByteCode>,
}

pub fn load(stream: &mut File) -> ParseProto {
    let mut constants = Vec::new();
    let mut byte_codes = Vec::new();
    let mut lex = Lexer::new(stream);

    loop {
        match lex.next() {
            Token::Name(name) => {
                // `Name LiteralString` as function call
                // Push function name to the constants
                constants.push(Value::String(name));
                // Push instructions to get function name from constants and push to stack at 0
                byte_codes.push(ByteCode::GetGlobal(0, (constants.len() - 1) as u8));

                // TODO before parsing the function argument as a string, we have to check if there
                // are arguments enclosed in parentheses.
                // >>> print(1.0)

                match lex.next() {
                    Token::ParL => {
                        match lex.next() {
                            Token::Integer(i) => {
                                constants.push(Value::Integer(i));
                                byte_codes
                                    .push(ByteCode::LoadConst(1, (constants.len() - 1) as u8));
                            }
                            _ => panic!("Expected something else"),
                        }
                        if let Token::ParR = lex.next() {
                        } else {
                            panic!("No closing paren")
                        }
                        byte_codes.push(ByteCode::Call(0, 1));
                    }
                    Token::String(s) => {
                        constants.push(Value::String(s));
                        byte_codes.push(ByteCode::LoadConst(1, (constants.len() - 1) as u8));
                        byte_codes.push(ByteCode::Call(0, 1));
                    }
                    _ => panic!("expected string"),
                }

                // Parse the string argument, save it to the constants.
                // Add instruction to load the argument from constants onto the stack on top of
                // the function call, then push a call instruction which is supposed to use the
                // function at index 0 and use index 1 as arguments
                // if let Token::String(s) = lex.next() {
                //     constants.push(Value::String(s));
                //     byte_codes.push(ByteCode::LoadConst(1, (constants.len() - 1) as u8));
                //     byte_codes.push(ByteCode::Call(0, 1));
                // } else {
                //     panic!("expected string");
                // }
            }
            Token::Eos => break,
            t => panic!("unexpected token: {t:?}"),
        }
    }

    dbg!(&constants);
    dbg!(&byte_codes);
    ParseProto {
        constants,
        byte_codes,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{self, Seek, Write};
    use tempfile::tempfile;

    fn prepare_file(code: &str) -> File {
        let mut file = tempfile().unwrap();
        file.write(code.as_bytes()).unwrap();
        file.seek(io::SeekFrom::Start(0)).unwrap();
        file
    }

    #[test]
    fn parse_print_hello_world() {
        let mut file = prepare_file("print \"hello world!\"\n");

        let proto = load(&mut file);

        assert_eq!(
            proto.constants,
            vec![
                Value::String("print".to_string()),
                Value::String("hello world!".to_string())
            ]
        );
    }

    #[test]
    fn parse_print_integer() {
        let mut file = prepare_file("print (1)");

        let proto = load(&mut file);

        assert_eq!(
            proto.constants,
            vec![Value::String("print".to_string()), Value::Integer(1)]
        );
    }
}
