#include <stdint.h>

#define UART_BASE 0x10000000
#define UART_LSR  (UART_BASE + 5) // Line Status Register
#define UART_THR  (UART_BASE + 0) // Transmit Holding Register
#define UART_LSR_THRE (1 << 5)    // Transmitter Holding Register Empty

void putchar(char c) {
    volatile char *uart_thr = (volatile char *)UART_THR;
    volatile char *uart_lsr = (volatile char *)UART_LSR;
    
    // Wait until UART is ready to transmit
    while (!(*uart_lsr & UART_LSR_THRE));
    
    *uart_thr = c;
}

void print_string(const char *str) {
    while (*str) {
        putchar(*str++);
    }
}

void main() {
    print_string("Hello, RISC-V!\n");
    while (1); // Infinite loop to prevent exit
}
