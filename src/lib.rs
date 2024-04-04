use std::{
    fs::File,
    io::{Stdout, Write},
};

mod bytecode;
mod lex;
mod parse;
mod value;
mod vm;

pub fn lua<'a>(input: File, output: &'a mut (dyn Write + 'a)) {
    let proto = parse::load(input);

    vm::ExeState::new(output).execute(&proto);
}
