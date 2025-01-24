#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum AssemblyCommand<'a> {
    SysCallWrite,                   // mov eax, 4 ;
    SysCallRead,                    // mov eax, 3 ;
    SysCallExit,                    // mov eax, 1 ;
    LoadImmediate(&'a str),         // mov eax, imm ;
    Add(&'a str, &'a str),          // add r0, r1 ;
    Subtract(&'a str, &'a str),     // sub r0, r1 ;
    Multiply(&'a str),              // mul r0 ;
    IMultiply(&'a str, &'a str),    // imul r0, r1 ;
    Divide(&'a str),                // idiv r0 ;
    Push(&'a str),                  // push r0 ;
    Pop(&'a str),                   // pop r0 ;
    Jump(&'a str),                  // jmp label ;
    Return,                         // ret ;
}

impl<'a> AssemblyCommand<'a> {
    pub fn to_str(&self) -> String {
        match self {
            Self::SysCallWrite => "     mov eax, 4 ;\n".to_string(),
            Self::SysCallRead => "      mov eax, 3 ;\n".to_string(),
            Self::SysCallExit => "      mov eax, 1 ;\n".to_string(),
            Self::LoadImmediate(imm) => format!("       mov eax, {} ;\n", imm),
            Self::Add(r0, r1) => format!("      add {}, {} ;\n", r0, r1),
            Self::Subtract(r0, r1) => format!("     sub {}, {} ;\n", r0, r1),
            Self::IMultiply(r0, r1) => format!("     imul {}, {} ;\n", r0, r1),
            Self::Multiply(r0) => format!("     mul {} ;\n", r0),
            Self::Divide(r0) => format!("       idiv {} ;\n", r0),
            Self::Push(r0) => format!("     push {} ;\n", r0),
            Self::Pop(r0) => format!("      pop {} ;\n", r0),
            Self::Jump(label) => format!("      jmp {} ;\n", label),
            Self::Return => "       ret \n;".to_string(),
        }
    }
}
