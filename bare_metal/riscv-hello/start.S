        .section .text
        .global _start

_start:
        # Set up stack pointer
        la sp, stack_top

        # Call main function in C
        call main

        # Infinite loop to prevent exit
1:      j 1b

        .section .bss
        .align 4
stack:
        .skip 1024           # 1 KB stack
stack_top:
