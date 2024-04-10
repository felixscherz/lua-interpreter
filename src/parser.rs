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
                constants.push(Value::String(name));
                byte_codes.push(ByteCode::GetGlobal(0, (constants.len() - 1) as u8));

                if let Token::String(s) = lex.next() {
                    constants.push(Value::String(s));
                    byte_codes.push(ByteCode::LoadConst(1, (constants.len() - 1) as u8));
                    byte_codes.push(ByteCode::Call(0, 1));
                } else {
                    panic!("expected string");
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