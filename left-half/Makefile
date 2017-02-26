all: build

clean:
	platformio run --target=clean
	rm -rf rust/target

build: build-rust build-arduino

build-arduino:
	platformio run

build-rust:
	cd rust; make

flash: build
	rsync -avc .pioenvs/tau/firmware.bin b:/tmp/firmware-left.bin && ssh b /home/debian/BOSSA/bin/bossac -i -d --port=ttyACM0 -e -w -v -b /tmp/firmware-left.bin -R

