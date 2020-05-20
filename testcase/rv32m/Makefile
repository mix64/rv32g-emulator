fib.text: rv32m.s
	riscv32-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o rv32m rv32m.s
	riscv32-unknown-elf-objcopy -O binary rv32m rv32m.text
	riscv32-unknown-elf-objdump -S rv32m > rv32m.asm

clean:
	rm -f rv32m rv32m.text rv32m.asm