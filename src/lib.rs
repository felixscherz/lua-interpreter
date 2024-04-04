use std::{fs::File, io::{Stdout, Write}};

mod value;
mod bytecode;
mod lex;
mod parse;
mod vm;

pub fn lua(input: File, stdout: Stdout) {
    let proto = parse::load(input);
    vm::ExeState::new(stdout).execute(&proto);
}
