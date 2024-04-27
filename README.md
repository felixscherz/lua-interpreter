# a lua interpreter in rust

based on [Build a Lua Interpreter in Rust](https://wubingzheng.github.io/build-lua-in-rust/en)
by [Wubing Zheng](https://github.com/WuBingzheng)


## How I think the stack is supposed to work with bytecode

Operations can manipulate in three ways:

1. push a value on top of the stack, incrementing stack size
2. pop a value off the stack
3. set the value of an item on the stack at a specific index


### bytecode instructions as rust enums

Bytecode instruction as specified in [luajit](https://luajit.org/):

```
A single bytecode instruction is 32 bit wide and has an 8 bit opcode field and
several operand fields of 8 or 16 bit. Instructions come in one of two formats:

+---+---+---+---+
| B | C | A | OP|
|   D   | A | OP|
+---+---+---+---+
```

## How local variables work

When assigning local variables they are stored on the stack and their name is appended to a `locals` list.
When a name is referenced, first go through `locals` in reverse. If the name can be found, its index points to the
stack index where the value is stored. The VM can then use the MOVE(dst, src) bytecode to copy that value to the top
of the stack.
