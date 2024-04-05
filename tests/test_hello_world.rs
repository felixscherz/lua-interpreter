use lua_interpreter::lua;
use std::io::{self, Read, Seek, Write};

use tempfile::tempfile;

#[test]
fn test_hello_world() {
    let mut file = tempfile().unwrap();
    file.write("print \"hello world!\"\n".as_bytes()).unwrap();
    file.seek(io::SeekFrom::Start(0)).unwrap();

    let mut output = tempfile().unwrap();

    lua(&mut file, &mut output);
    let mut buffer = String::new();
    output.seek(io::SeekFrom::Start(0)).unwrap();
    output.read_to_string(&mut buffer).unwrap();
    assert_eq!(buffer, "hello world!\n");
}
