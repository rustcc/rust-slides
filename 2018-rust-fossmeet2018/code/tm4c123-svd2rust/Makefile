
debug:
	xargo build
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/tm4c123-svd2rust upload/tm4c123-svd2rust.debug.bin

release:
	xargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/tm4c123-svd2rust upload/tm4c123-svd2rust.release.bin

flash_release:
	sudo lm4flash upload/tm4c123-svd2rust.release.bin

clean:
	xargo clean
	rm upload/*
