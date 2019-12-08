
debug:
	xargo build
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/stm32f3-hal1 upload/stm32f3-hal1.debug.bin

release:
	xargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/stm32f3-hal1 upload/stm32f3-hal1.release.bin


flash_release:
	st-flash write upload/stm32f3-hal1.release.bin 0x8000000  

clean:
	xargo clean
	rm upload/*
