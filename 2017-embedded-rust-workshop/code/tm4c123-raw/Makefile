
debug:
	xargo build
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/tm4c123-raw upload/tm4c123-raw.debug.bin

release:
	xargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/tm4c123-raw upload/tm4c123-raw.release.bin

flash_debug:
	sudo lm4flash upload/tm4c123-raw.debug.bin

flash_release:
	sudo lm4flash upload/tm4c123-raw.release.bin
		   
clean:
	xargo clean
	rm upload/*
