//! Basic bare metal image for qemu RISC-V image
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;
use core::arch::global_asm;
use core::fmt::Write;
use arrayvec::ArrayString;

// Include startup assembly here so it is linked in
global_asm!(include_str!("startup.s"));

/// UART0 base address on the QEMU virt machine
const UART0: *mut u8 = 0x1000_0000 as *mut u8;

/// Write a single byte to UART0.
fn uart_put(c: u8) {
    unsafe {
        ptr::write_volatile(UART0, c);
    }
}

/// Write a string to UART0.
fn uart_print(s: &str) {
    for b in s.bytes() {
        uart_put(b);
    }
}

/// Since we expact this image to run in qemu, write the special
/// memory location to exit the simulator
fn exit_qemu()
{
    // Gracefully shut down QEMU via the SiFive test device (virt machine).
    // Writing 0x5555 to 0x100000 triggers a clean exit.
    const VIRT_TEST: *mut u32 = 0x10_0000 as *mut u32;
    unsafe {
        ptr::write_volatile(VIRT_TEST, 0x5555);
    }
}
/// Entry point — called from the assembly startup code.
#[no_mangle]
pub extern "C" fn main() -> ! {
    if cfg!(feature = "test_panic") {
        panic!("Test panic");
    }

    uart_print("Hello world!\n");
    exit_qemu();

    // Fallback: spin forever if shutdown didn't work.
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    uart_print("PANIC: ");
    if let Some(location) = info.location() {
        uart_print(location.file());
        uart_print(": ");

        // Array string is stack allocated string.  11 is
        // the max size of a u32 string representation
        let mut line_number_buf = ArrayString::<11>::new();
        write!(&mut line_number_buf, "{}", location.line()).unwrap();
        let line_num_as_str = line_number_buf.as_str();
        uart_print(line_num_as_str);
    }
    uart_print("\n");

    if cfg!(feature = "test_panic") {
        exit_qemu();
    }

    loop {}
}
