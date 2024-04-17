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

                // Parse the string argument, save it to the constants.
                // Add instruction to load the argument from constants onto the stack on top of
                // the function call, then push a call instruction which is supposed to use the
                // function at index 0 and use index 1 as arguments
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
