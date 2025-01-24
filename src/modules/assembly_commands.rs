#[derive(Debug, PartialEq)]
pub enum AssemblyCommand<'a> {
    SysCallWrite,             // mov eax, 4 ;
    SysCallRead,              // mov eax, 3 ;
    SysCallExit,              // mov eax, 1 ;
    LoadImmediate(&'a str),   // mov eax, imm ;
    Add(&'a str, &'a str),    // add r0, r1 ;
    Subtract(&'a str, &'a str), // sub r0, r1 ;
    Multiply(&'a str, &'a str), // imul r0, r1 ;
    Divide(&'a str),          // idiv r0 ;
    Push(&'a str),            // push r0 ;
    Pop(&'a str),             // pop r0 ;
    Jump(&'a str),            // jmp label ;
    Return,                   // ret ;
}

impl<'a> AssemblyCommand<'a> {
    pub fn to_str(&self) -> String {
        match self {
            Self::SysCallWrite => "mov eax, 4 ;".to_string(),
            Self::SysCallRead => "mov eax, 3 ;".to_string(),
            Self::SysCallExit => "mov eax, 1 ;".to_string(),
            Self::LoadImmediate(imm) => format!("mov eax, {} ;", imm),
            Self::Add(r0, r1) => format!("add {}, {} ;", r0, r1),
            Self::Subtract(r0, r1) => format!("sub {}, {} ;", r0, r1),
            Self::Multiply(r0, r1) => format!("imul {}, {} ;", r0, r1),
            Self::Divide(r0) => format!("idiv {} ;", r0),
            Self::Push(r0) => format!("push {} ;", r0),
            Self::Pop(r0) => format!("pop {} ;", r0),
            Self::Jump(label) => format!("jmp {} ;", label),
            Self::Return => "ret ;".to_string(),
        }
    }
}
