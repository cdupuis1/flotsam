//
// main.c
//
// Contains C entry point main
//
// https://github.com/freedomtan/aarch64-bare-metal-qemu/blob/master/test64.c as reference

// hw/arm/virt.c for qemu memory map
#define UART0_ADDR      0x09000000

void puts_uart(const char *theString)
{
    volatile unsigned long *uart0_reg;

    // Set the address of our pointer
    uart0_reg = (unsigned long *)UART0_ADDR;

    while (*theString != '\0') {
        *uart0_reg = *theString;
        theString++; // Go to next string
    }
}

void main()
{
    puts_uart("Hello World\n");
}