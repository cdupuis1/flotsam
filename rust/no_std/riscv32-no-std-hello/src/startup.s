//! Startup assembly to jump to rust code
//!
//! TODO: Add copy bss and data sections
.section .text.init
.global _start
_start:
    la sp, __stack_top
    call main
1:
    wfi
    j 1b