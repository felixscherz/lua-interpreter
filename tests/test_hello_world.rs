use lua_interpreter::lua;
use std::{
    fs::File,
    io::{self, Read, Seek, Write},
};

use tempfile::tempfile;

fn prepare_file(code: &str) -> File {
    let mut file = tempfile().unwrap();
    file.write(code.as_bytes()).unwrap();
    file.seek(io::SeekFrom::Start(0)).unwrap();
    file
}

fn compare_output(output: &mut File, expected: &str) {
    let mut buffer = String::new();
    output.seek(io::SeekFrom::Start(0)).unwrap();
    output.read_to_string(&mut buffer).unwrap();
    assert_eq!(buffer, expected);
}

#[test]
fn test_hello_world() {
    let mut file = prepare_file("print \"hello world!\"\n");
    let mut output = tempfile().unwrap();

    lua(&mut file, &mut output);

    compare_output(&mut output, "hello world!\n");
}

#[test]
fn test_print_integer() {
    let mut file = prepare_file("print(1)\n");
    let mut output = tempfile().unwrap();

    lua(&mut file, &mut output);

    compare_output(&mut output, "1\n");
}

#[test]
fn test_print_bool() {
    let mut file = prepare_file("print(true)\n");
    let mut output = tempfile().unwrap();

    lua(&mut file, &mut output);

    compare_output(&mut output, "true\n");
}

#[test]
fn test_print_nil() {
    let mut file = prepare_file("print(nil)\n");
    let mut output = tempfile().unwrap();

    lua(&mut file, &mut output);

    compare_output(&mut output, "nil\n");
}

#[test]
fn test_print_local_variable() {
    let mut file = prepare_file("local a = 1\nprint(a)");
    let mut output = tempfile().unwrap();

    lua(&mut file, &mut output);

    compare_output(&mut output, "1\n");
}
