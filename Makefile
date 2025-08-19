default:
	st-flash --reset write ./bootloader/bin_files/boot.bin 0x08000000
	st-flash --reset write ./kernel/bin_files/kernel.bin   0x08020000
