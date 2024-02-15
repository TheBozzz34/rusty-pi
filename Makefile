default:
	rm -rf target
	cargo rustc -- -C link-arg=--script=./linker.ld
	arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/rusty-pi ./kernel7.img