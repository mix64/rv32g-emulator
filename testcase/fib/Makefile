fib.text: fib.c
	riscv32-unknown-elf-gcc -S fib.c
	riscv32-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o fib fib.s
	riscv32-unknown-elf-objcopy -O binary fib fib.text
	riscv32-unknown-elf-objdump -S fib > fib.asm

clean:
	rm -f fib fib.text fib.asm fib.s