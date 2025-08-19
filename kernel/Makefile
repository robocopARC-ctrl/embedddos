default:
	cargo build --release
	arm-none-eabi-strip ./target/thumbv7em-none-eabihf/release/kernel
	arm-none-eabi-objcopy ./target/thumbv7em-none-eabihf/release/kernel -O binary ./bin_files/kernel.bin

clean:
	cargo clean

flash:
	st-flash --reset write ./bin_files/kernel.bin 0x0802000
