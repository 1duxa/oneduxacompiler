global _start
_start:
     mov rax, 1
     push rax
     mov rax, 514
     push rax
     pop rax
     pop rbx
     add rax, rbx
     push rax
     mov rax, 1
     push rax
     mov rax, 5
     push rax
     push QWORD [rsp + 8]
     pop rax
     pop rbx
     div rbx
     push rax
     pop rax
     pop rbx
     add rax, rbx
     push rax
     mov rax, 60
     pop rdi
     syscall