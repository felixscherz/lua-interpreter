#[derive(Debug)]
pub enum ByteCode {
    /// GetGlobal(dst, src):
    /// load global name from constants at src and load global into stack at dst
    GetGlobal(u8, u8),
    /// LoadConst(dst, src):
    /// load value from constants into stack at dst
    LoadConst(u8, u8),
    Call(u8, u8),
}
