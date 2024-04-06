use std::{
    fs::File,
    io::{Write},
};

mod bytecode;
mod lexer;
mod parser;
mod value;
mod vm;

pub fn lua<'a>(input: &mut File, output: &'a mut (dyn Write + 'a)) {
    let proto = parser::load(input);

    vm::ExeState::new(output).execute(&proto);
}
