use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum AssemblyCommand {
    SysCallWrite,    // mov eax, 4 ;
    SysCallRead,     // mov eax, 3 ;
    SysCallExit,     // mov eax, 1 ;
    LoadImmediate,   // mov eax, imm ;
    Add,             // add eax, ebx ;
    Subtract,        // sub eax, ebx ;
    Multiply,        // imul eax, ebx ;
    Divide,          // idiv ebx ;
    Push,            // push eax ;
    Pop,             // pop eax ;
    Jump,            // jmp label ;
    Call,            // call label ;
    Return,          // ret ;
}

impl AssemblyCommand {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::SysCallWrite => "mov eax, 4 ;",
            Self::SysCallRead => "mov eax, 3 ;",
            Self::SysCallExit => "mov eax, 1 ;",
            Self::LoadImmediate => "mov eax, imm ;",
            Self::Add => "add eax, ebx ;",
            Self::Subtract => "sub eax, ebx ;",
            Self::Multiply => "imul eax, ebx ;",
            Self::Divide => "idiv ebx ;",
            Self::Push => "push eax ;",
            Self::Pop => "pop eax ;",
            Self::Jump => "jmp label ;",
            Self::Call => "call label ;",
            Self::Return => "ret ;",
        }
    }
}
