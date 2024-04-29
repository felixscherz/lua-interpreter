use crate::bytecode::ByteCode;
use crate::parser::ParseProto;
use crate::value::Value;
use std::collections::HashMap;
use std::io::Write;

pub struct ExeState<'a> {
    globals: HashMap<String, Value>,
    stack: Vec<Value>,
    output: &'a mut (dyn Write + 'a),
    func_index: usize,
}

impl<'a> ExeState<'a> {
    pub fn new(output: &'a mut (dyn Write + 'a)) -> Self {
        let mut globals = HashMap::new();
        globals.insert(String::from("print"), Value::Function(lib_print));

        ExeState {
            globals,
            stack: Vec::new(),
            output,
            func_index: 0,
        }
    }

    fn set_stack(&mut self, dst: u8, c: Value) {
        let dst: usize = dst.into();
        match dst.cmp(&self.stack.len()) {
            std::cmp::Ordering::Less => self.stack[dst] = c,
            std::cmp::Ordering::Equal => self.stack.push(c),
            std::cmp::Ordering::Greater => panic!("failed to set_stack"),
        }
    }

    fn get_stack(&self, src: u8) -> Value {
        self.stack.get(src as usize).unwrap().clone()
    }

    pub fn execute(&mut self, proto: &ParseProto) {
        for code in proto.byte_codes.iter() {
            dbg!(&self.stack);
            match *code {
                ByteCode::GetGlobal(dst, name) => {
                    let name = &proto.constants[name as usize];
                    if let Value::String(key) = name {
                        let v = self.globals.get(key).unwrap_or(&Value::Nil).clone();
                        self.set_stack(dst, v);
                    } else {
                        panic!("invalid global key: {name:?}");
                    }
                }
                ByteCode::LoadConst(dst, c) => {
                    let v = proto.constants[c as usize].clone();
                    self.set_stack(dst, v);
                }
                ByteCode::Call(func, _) => {
                    self.func_index = func as usize;
                    let func = &self.stack[self.func_index as usize];
                    if let Value::Function(f) = func {
                        f(self);
                    } else {
                        panic!("invalid function: {func:?}");
                    }
                }
                ByteCode::LoadNil(dst) => self.set_stack(dst, Value::Nil),
                ByteCode::LoadBool(dst, v) => self.set_stack(dst, Value::Boolean(v)),
                ByteCode::LoadInteger(dst, v) => self.set_stack(dst, Value::Integer(v.into())),
                ByteCode::Move(dst, src) => self.set_stack(dst, self.get_stack(src)),
            }
        }
    }
}

// "print" function in Lua's std-lib.
// It supports only 1 argument and assumes the argument is at func_index + 1 on stack.
fn lib_print(state: &mut ExeState) -> i32 {
    writeln!(state.output, "{:?}", state.stack[state.func_index + 1]).unwrap();
    0
}

#[cfg(test)]
mod test {
    use std::{
        fs::File,
        io::{self, Read, Seek, Write},
    };

    use tempfile::tempfile;

    use crate::parser::load;

    use super::ExeState;

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
        let proto = load(&mut file);

        let mut vm = ExeState::new(&mut output);
        vm.execute(&proto);

        compare_output(&mut output, "hello world!\n");
    }

    #[test]
    fn test_print_small_integer() {
        let mut file = prepare_file("print(1)");
        let mut output = tempfile().unwrap();
        let proto = load(&mut file);

        let mut vm = ExeState::new(&mut output);
        vm.execute(&proto);

        compare_output(&mut output, "1\n");
    }

    #[test]
    fn parse_print_large_integer() {
        let mut file = prepare_file("print(33000)");
        let mut output = tempfile().unwrap();
        let proto = load(&mut file);

        let mut vm = ExeState::new(&mut output);
        vm.execute(&proto);

        compare_output(&mut output, "33000\n");
    }

    #[test]
    fn parse_print_float() {
        let mut file = prepare_file("print(1.5)");
        let mut output = tempfile().unwrap();
        let proto = load(&mut file);

        let mut vm = ExeState::new(&mut output);
        vm.execute(&proto);

        compare_output(&mut output, "1.5\n");
    }

    #[test]
    fn assign_local_variable_then_print() {
        let mut file = prepare_file("local a = 1\nprint(a)");
        let mut output = tempfile().unwrap();
        let proto = load(&mut file);

        let mut vm = ExeState::new(&mut output);
        vm.execute(&proto);

        compare_output(&mut output, "1\n");
    }
}
