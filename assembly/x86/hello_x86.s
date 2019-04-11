;
; hello.s
;
; Hello world assembly program for x86
; Take from: https://jameshfisher.com/2018/03/10/linux-assembly-hello-world.html
;
; To build:
;
; nasm -f elf64 -o hello_x86.o hello_x86.s
; ld -o hello_x86 hello_x86.o
;
global _start

section .text

_start:
  mov rax, 1        ; write(
  mov rdi, 1        ;   STDOUT_FILENO,
  mov rsi, msg      ;   "Hello, world!\n",
  mov rdx, msglen   ;   sizeof("Hello, world!\n")
  syscall           ; );

  mov rax, 60       ; exit(
  mov rdi, 0        ;   EXIT_SUCCESS
  syscall           ; );

section .rodata
  msg: db "Hello, world!", 10
  msglen: equ $ - msg
