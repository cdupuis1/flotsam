//
// start.S
//
// Assembly file to define stack and branch to C entry point
//
.global _start
_start:
	ldr x30, =stack_top //Stack top address defiend in linker script
	mov sp, x30 //Set the stack pointer register
	bl main //main will be our C entry point
	b . //Infinite loop once we come back from C 
