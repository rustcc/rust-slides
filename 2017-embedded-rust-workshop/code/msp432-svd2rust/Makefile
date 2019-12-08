
debug:
	xargo build
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/msp432-svd2rust upload/msp432-svd2rust.debug.bin

release:
	xargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/msp432-svd2rust upload/msp432-svd2rust.release.bin


clean:
	xargo clean
	rm upload/*
