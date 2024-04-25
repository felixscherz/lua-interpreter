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
                let src = add_const(&mut constants, Value::String(name));
                // Push instructions to get function name from constants and push to stack at 0
                byte_codes.push(ByteCode::GetGlobal(0, src as u8));
                match lex.next() {
                    Token::ParL => {
                        match lex.next() {
                            Token::Integer(i) => {
                                if let Ok(smallint) = i16::try_from(i) {
                                    byte_codes.push(ByteCode::LoadInteger(1, smallint));
                                } else {
                                    byte_codes.push(load_const(
                                        &mut constants,
                                        1,
                                        Value::Integer(i),
                                    ))
                                }
                            }
                            Token::Float(f) => {
                                byte_codes.push(load_const(&mut constants, 1, Value::Float(f)))
                            }
                            Token::Nil => {
                                byte_codes.push(ByteCode::LoadNil(1));
                            }
                            Token::True => byte_codes.push(ByteCode::LoadBool(1, true)),
                            Token::False => byte_codes.push(ByteCode::LoadBool(1, false)),
                            _ => panic!("Expected something else"),
                        }
                        if let Token::ParR = lex.next() {
                        } else {
                            panic!("No closing paren")
                        }
                        byte_codes.push(ByteCode::Call(0, 1));
                    }
                    Token::String(s) => {
                        let src = add_const(&mut constants, Value::String(s));
                        byte_codes.push(ByteCode::LoadConst(1, src as u8));
                        byte_codes.push(ByteCode::Call(0, 1));
                    }
                    _ => panic!("expected string"),
                }
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

/// adding constants is a separate function because it is a place to make performance optimizations
/// in case of duplicate constants we can just reference the same value instead of adding it twice
fn add_const(constants: &mut Vec<Value>, v: Value) -> usize {
    constants.push(v);
    constants.len() - 1
}

fn load_const(constants: &mut Vec<Value>, dst: usize, v: Value) -> ByteCode {
    ByteCode::LoadConst(dst as u8, add_const(constants, v) as u8)
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
    fn parse_print_large_integer() {
        let mut file = prepare_file("print(33000)");

        let proto = load(&mut file);

        assert_eq!(
            proto.constants,
            vec![Value::String("print".to_string()), Value::Integer(33000)]
        );
    }

    #[test]
    fn parse_print_small_integer() {
        let mut file = prepare_file("print(1)");

        let proto = load(&mut file);

        assert_eq!(
            proto.byte_codes,
            vec![
                ByteCode::GetGlobal(0, 0),
                ByteCode::LoadInteger(1, 1_i16),
                ByteCode::Call(0, 1)
            ]
        );
    }

    #[test]
    fn parse_print_float() {
        let mut file = prepare_file("print(1.5)");

        let proto = load(&mut file);

        assert_eq!(
            proto.constants,
            vec![Value::String("print".to_string()), Value::Float(1.5)]
        );
    }
}
