all:
	riscv64-unknown-elf-as -march=rv64gc -mabi=lp64d -o start.o start.S
	# Need to include mcmodel=medany as the starting address is above 2GB
	riscv64-unknown-elf-gcc -c -o main.o main.c -march=rv64gc -mabi=lp64d -mcmodel=medany
	riscv64-unknown-elf-ld -Tlinker.ld -o program.elf start.o main.o

clean:
	rm -f start.o main.o program.elf

qemu: clean all
	qemu-system-riscv64 -machine virt -nographic -bios none -kernel program.elf

