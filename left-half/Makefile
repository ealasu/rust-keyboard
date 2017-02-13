PROG=target/thumbv7em-none-eabi/release/keyboard-left-half
OBJCOPY=arm-none-eabi-objcopy
DFUUTIL?=	dfu-util
DFUVID?=	2323
DFUPID?=	0001

all: clean ${PROG}.bin ${PROG}.hex

clean:
	rm -f ${PROG} ${PROG}.hex ${PROG}.bin

${PROG}:
	xargo build --release

%.bin: ${PROG}
	${OBJCOPY} -O binary $< $@

%.hex: ${PROG}
	${OBJCOPY} -O ihex $< $@

flash: clean ${PROG}.bin
	${DFUUTIL} -d ${DFUVID}:${DFUPID} -D ${PROG}.bin
