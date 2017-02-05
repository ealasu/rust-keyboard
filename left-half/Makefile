build:
	@xargo build --release

flash: build
	arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabi/release/left-half target/hex
	~/.platformio/packages/tool-teensy/teensy_loader_cli -w -s -mmcu=mk20dx256 target/hex

clean:
	xargo clean
	git clean -Xf
