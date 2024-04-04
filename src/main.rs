use std::env;
use std::fs::File;
use std::io::stdout;

mod bytecode;
mod lex;
mod parse;
mod value;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} script", args[0]);
        return;
    }
    let file = File::open(&args[1]).unwrap();

    let proto = parse::load(file);
    vm::ExeState::new(&mut stdout()).execute(&proto);
}
