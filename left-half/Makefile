all: build

clean:
	platformio run --target=clean

build: build-rust build-arduino

build-arduino:
	platformio run

build-rust:
	cd rust; make

flash: build
	rsync -avc .pioenvs/tau/firmware.bin b:~/ && ssh b /home/debian/BOSSA/bin/bossac -i -d --port=ttyACM0 -e -w -v -b /home/debian/firmware.bin -R

