// Build with:
//
// as hello_aarch64.s -o hello_aarch64.o
// ld hello_aarch64.o -o hello_aarch64
.text
.globl _start

_start:
	mov x0, #1 // stdout is file descriptor 1
	adrp x1, message // Make PC relative address
	add x1, x1, :lo12:message // Not quite sure what this does
	mov x2, message_len // Number of bytes to write
	mov x8, #64 // Write system call
	svc #0 // Invoke system call
	mov x0, #0 // Exit code
	mov x8, #93 // Exit system call
	svc #0
.data

message:	.asciz "Hello World\n"
message_len =   . - message
