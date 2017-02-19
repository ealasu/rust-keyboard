#define KINETISK
#define __MK20DX128__
#define TEENSYDUINO 118
#define F_CPU 48000000
#define USB_SERIAL
#define LAYOUT_US_ENGLISH
#define KEYBOARD_INTERFACE
#define KEYMEDIA_INTERFACE

#define isascii(__c)    ((unsigned)(__c)<=0177)
#define toascii(__c)    ((__c)&0177)

include "usb_keyboard.h"
