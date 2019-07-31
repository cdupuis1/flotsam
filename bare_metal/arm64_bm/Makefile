all:
	aarch64-none-elf-as -g start.S -o start.o
	aarch64-none-elf-gcc -c -g main.c -o main.o
	aarch64-none-elf-ld -T sample.ld main.o start.o -o main.elf
	aarch64-none-elf-objcopy -O binary main.elf main.bin

clean:
	rm -f *.o *.bin *.elf
