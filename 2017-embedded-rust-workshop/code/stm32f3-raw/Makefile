
debug:
	xargo build
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/stm32f3-raw upload/stm32f3-raw.debug.bin

release:
	xargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/stm32f3-raw upload/stm32f3-raw.release.bin


flash_release:
	st-flash write upload/stm32f3-raw.release.bin 0x8000000  

flash_debug:
	st-flash write upload/stm32f3-raw.debug.bin 0x8000000  
    
clean:
	xargo clean
	rm upload/*
