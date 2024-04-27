use crate::bytecode::ByteCode;
use crate::parser::ParseProto;
use crate::value::Value;
use std::collections::HashMap;
use std::io::Write;

pub struct ExeState<'a> {
    globals: HashMap<String, Value>,
    stack: Vec<Value>,
    output: &'a mut (dyn Write + 'a),
}

impl<'a> ExeState<'a> {
    pub fn new(output: &'a mut (dyn Write + 'a)) -> Self {
        let mut globals = HashMap::new();
        globals.insert(String::from("print"), Value::Function(lib_print));

        ExeState {
            globals,
            stack: Vec::new(),
            output,
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
                    let func = &self.stack[func as usize];
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
// It supports only 1 argument and assumes the argument is at index:1 on stack.
fn lib_print(state: &mut ExeState) -> i32 {
    writeln!(state.output, "{:?}", state.stack[1]).unwrap();
    0
}
