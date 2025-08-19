default:
	cargo build --release
	arm-none-eabi-strip ./target/thumbv7em-none-eabihf/release/bootloader
	arm-none-eabi-objcopy ./target/thumbv7em-none-eabihf/release/bootloader -O binary ./bin_files/boot.bin
