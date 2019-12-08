
debug:
	xargo build
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/stm32f3-svd2rust upload/stm32f3-svd2rust.debug.bin

release:
	xargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/stm32f3-svd2rust upload/stm32f3-svd2rust.release.bin


flash_release:
	st-flash write upload/stm32f3-svd2rust.release.bin 0x8000000  

flash_debug:
	st-flash write upload/stm32f3-svd2rust.debug.bin 0x8000000  
    
clean:
	xargo clean
	rm upload/*
