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

#[test]
fn test_hello_world() {
    let mut file = prepare_file("print \"hello world!\"\n");
    let mut output = tempfile().unwrap();

    lua(&mut file, &mut output);

    let mut buffer = String::new();
    output.seek(io::SeekFrom::Start(0)).unwrap();
    output.read_to_string(&mut buffer).unwrap();
    assert_eq!(buffer, "hello world!\n");
}

#[test]
fn test_print_integer() {
    let mut file = prepare_file("print(1)\n");
    let mut output = tempfile().unwrap();

    lua(&mut file, &mut output);

    let mut buffer = String::new();
    output.seek(io::SeekFrom::Start(0)).unwrap();
    output.read_to_string(&mut buffer).unwrap();
    assert_eq!(buffer, "1\n");
}

#[test]
fn test_print_bool() {
    let mut file = prepare_file("print(true)\n");
    let mut output = tempfile().unwrap();

    lua(&mut file, &mut output);

    let mut buffer = String::new();
    output.seek(io::SeekFrom::Start(0)).unwrap();
    output.read_to_string(&mut buffer).unwrap();
    assert_eq!(buffer, "true\n");
}

#[test]
fn test_print_nil() {
    let mut file = prepare_file("print(nil)\n");
    let mut output = tempfile().unwrap();

    lua(&mut file, &mut output);

    let mut buffer = String::new();
    output.seek(io::SeekFrom::Start(0)).unwrap();
    output.read_to_string(&mut buffer).unwrap();
    assert_eq!(buffer, "nil\n");
}
