use std::{
    fs::File,
    io::{Stdout, Write},
};

mod bytecode;
mod lexer;
mod parser;
mod value;
mod vm;

pub fn lua<'a>(input: File, output: &'a mut (dyn Write + 'a)) {
    let proto = parser::load(input);

    vm::ExeState::new(output).execute(&proto);
}
