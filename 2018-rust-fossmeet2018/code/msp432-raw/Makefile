
debug:
	xargo build
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/msp432-raw upload/msp432-raw.debug.bin

release:
	xargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/msp432-raw upload/msp432-raw.release.bin

		   
clean:
	xargo clean
	rm upload/*
