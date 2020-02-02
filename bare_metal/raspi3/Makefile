all:
	aarch64-none-elf-as -c boot.S -o boot.o
	aarch64-none-elf-gcc -ffreestanding -c kernel.c -o kernel.o -O2 -Wall -Wextra
	aarch64-none-elf-gcc -T raspi.lds -o kernel.elf -ffreestanding -O2 -nostdlib boot.o kernel.o -lgcc
	aarch64-none-elf-objcopy kernel.elf -O binary kernel8.img

clean:
	rm -f boot.o kernel.o kernel.elf kernel8.img

run:
	qemu-system-aarch64 -m 1024 -M raspi3 -serial stdio -kernel kernel.elf