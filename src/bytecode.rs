#[derive(Debug, PartialEq)]
pub enum ByteCode {
    /// GetGlobal(dst, src):
    /// load global name from constants at src and load global into stack at dst
    GetGlobal(u8, u8),
    /// LoadConst(dst, src):
    /// load value from constants into stack at dst
    LoadConst(u8, u8),
    /// Call(func_index, ???)
    /// invokes the function at funx_index on the stack, for now it expects a single argument
    /// at func_index+1 on the stack
    Call(u8, u8),
    LoadBool(u8, bool),
    LoadNil(u8),
    /// LoadInteger(dst, value)
    /// Since bytecodes can be at most 4 bytes and one byte is reserved for the enum tag
    /// we can get away with storing small integers directly in the bytecode instruction
    LoadInteger(u8, i16),
    /// Move(dst, src)
    Move(u8, u8),
}
