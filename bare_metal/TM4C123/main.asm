;
; main.asm
;
; Assembly code to turn red LED on continuously on TM4C123 launchpad

; .data section is initialized data
.data

; We have to declare the register addresses as variables as we can only pass
; 16-bit immediate values
SYSCTL_RCGCGPIO_R .word 0x400FE608
GPIO_PORTF_DIR_R  .word 0x40025400
GPIO_PORTF_DEN_R  .word 0x4002551C
GPIO_PORTF_DATA_R .word 0x400253FC

; Program code goes here
.text

	; Export main so it can be called by C files
	.global main
main:
	bl		gpio_init

loop    bl		light_on
        b		loop

; Initialize the GPIO pins that control the red LED
gpio_init:
	; Enable the clock for GPIO port B
	ldr r1, SYSCTL_RCGCGPIO_R
	ldr r0,[r1]
	orr r0,r0,#0x20
	str r0,[r1]

	; Set the direction of pin 1 to out
	ldr r1, GPIO_PORTF_DIR_R
	mov r0,#0x02  ;0B 0000 0010
	str r0,[r1]

	; Enable the digital enable for port 1
	ldr r1, GPIO_PORTF_DEN_R
	mov r0,#0x02
	str r0,[R1]
	bx	lr

light_on:
	; Set the RED LED on
	ldr r1, GPIO_PORTF_DATA_R
	mov r0, #0x02
	str r0,[r1]
	bx lr

.end
