;
; main.asm
;
; Assembly code to turn the red LED on when first switch is pressed
;
; Based on code from https://cortex-m.com/archive/

.data

SYSCTL_RCGCGPIO_R  .word 0x400FE608

; The way the GPIO register addressing works is that each port has a five
; hex digit base and then each port has the same offset scheme (see
; table Table 10-6. GPIO Register Map in the data sheet).  The base for
; port F is 0x40025 (see section 10-4 in data sheet)
GPIO_PORTF_DATA_R  .word 0x400253FC
GPIO_PORTF_DIR_R   .word 0x40025400
GPIO_PORTF_DEN_R   .word 0x4002551C

GPIO_PORTF_PUR_R   .word 0x40025510
GPIO_PORTF_LOCK_R  .word 0x40025520
GPIO_PORTF_CR_R	   .word 0x40025524
; Magic value that unlocks the commit register
GPIO_LOCK_KEY	   .word 0x4C4F434B

; The delay is simply doing a subtract and compare and I'm not quite sure how
; many CPUs cycles doing those two operations is and then dividing that into
; the 80Mhz system clock.  Will presume this is a magic value
SEC_DIV_FIVE	.word 1066666 ; 1 second divided by 5

;switch1 = PF4
;switch2 = PF0
RED		.word 0x02 ; 0b  0000 0010

.text

	.global	main

main:
	bl	gpio_init
main_loop:
	ldr r0, SEC_DIV_FIVE
	bl delay
	bl input ; Get status of both switch 1 and switch 2
	cmp r0, #0x01 ; Port 4 will be 0 if switch 1 is pressed
	beq switch_on
	; We end up writing 0x01 to the GPIO data register which will turn off
	; the red LED
	bl output
    b main_loop


; Initialize GPIO register for use
gpio_init:
	; Enable the clock GPIO port F
	ldr r1, SYSCTL_RCGCGPIO_R
	ldr r0, [r1]
	orr r0, r0, #0x20
	str r0, [r1]

	; Disables write lock on the GPIO commit register
	ldr r1, GPIO_PORTF_LOCK_R
	ldr r0, GPIO_LOCK_KEY
	str r0, [r1]

	; Set all the bits in the GPIO commit register so that when a value is
	; written to subsequent registers all bits will be allowed to be written
	ldr r1,GPIO_PORTF_CR_R
	mov r0, #0xFF
	str r0,[r1]

	; Set direction of GPIO pin for port F to output
	ldr r1, GPIO_PORTF_DIR_R
	mov r0, #0x0E   ; 0b 0000 1110
	str r0,[r1]

	; Enable weak pull up resistor for port F since this is a switch
	ldr r1, GPIO_PORTF_PUR_R
	mov r0, #0x10
	str r0,[r1]

	; Set the digital enable pin for port F.  By default a pin cannot be used
	; to drive a signal for logic (i.e. be on or off) so this functionality
	; must be explicitly enabled
	ldr r1, GPIO_PORTF_DEN_R
	mov r0, #0x1F   ;0B 0001 1111
	str r0,[r1]

	bx	lr

; Simple countdown delay where we keep subtracting till we hit zero
delay:
    subs  r0, r0, #1
	bne   delay
	bx    lr

input:
	ldr r1,GPIO_PORTF_DATA_R
	ldr r0, [r1]
	; r0 is both the source and destination as the result of r0 & 0x11 is
	; stored back in r0.  Bits 0 and 4 tell us if one of the buttons have
	; been pressed.  The buttons are logic low buttons so that if they are
	; zero then they are activated
	and r0, r0, #0x11 ; 0B 0001 0001
	bx lr

output:
	ldr r1, GPIO_PORTF_DATA_R
    str r0, [r1]
    BX  LR

switch_on:
	; We can write a 0x2 here as only bits 2, 3 and 4 are configured for
	; output.  According to the data sheet if a bit is not set for output
	; then anything written to it would be ignored.  In this case bit 0
	; and 4 are input to tell us the state of the switch so writing a 0
	; to the register for that bit will not affect the switches.
	ldr r0, RED
    bl output
    b main_loop ; Go back to the beginning of the main loop

.end
