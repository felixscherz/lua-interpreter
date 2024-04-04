use std::io::{self, Write};
use lua_interpreter::lua;


use tempfile::tempfile;

#[test]
fn test_hello_world() {
    let mut file  = tempfile().unwrap();
    file.write("print \"hello world\"".as_bytes()).unwrap();
    lua(file, io::stdout());
    assert!(true)
}
