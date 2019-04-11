#
# hello_arm.s
#
# Hello world program in ARM assembly
#
# Taken from: http://kerseykyle.com/articles/ARM-assembly-hello-world
#
# To build:
#
# as hello_arm.s -o hello_arm.o
# ld hello_arm.o -o hello_arm
.text            
.global _start
_start:
    mov r0, #1
    ldr r1, =message
    ldr r2, =len
    mov r7, #4
    swi 0

    mov r7, #1
    swi 0

.data
message:
    .asciz "hello world\n"
len = .-message 
