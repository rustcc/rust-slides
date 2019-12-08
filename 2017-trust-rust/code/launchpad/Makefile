
debug:
	xargo build
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/launchpad upload/launchpad.debug.bin

release:
	xargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/launchpad upload/launchpad.release.bin

flash:
	sudo lm4flash upload/launchpad.debug.bin

clean:
	xargo clean
	rm upload/*
