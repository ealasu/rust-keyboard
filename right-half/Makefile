all: build

build: build-rust build-arduino

build-arduino:
	platformio run

build-rust:
	cd kbd; make

flash: build
	#platformio run --target upload
	rsync -avc --progress .pioenvs/lpmsp430g2553/firmware.hex b:/tmp/ && ssh root@b mspdebug rf2500 --force-reset '"prog /tmp/firmware.hex"'

