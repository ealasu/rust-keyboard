#include "kbd.h"

void setup() {
  Serial.begin(9600);
  Serial1.begin(9600);
  while (!Serial) ;
}

void loop() {
  kbd_run_loop();
}

extern "C" int8_t serial_write(uint8_t b) {
  return Serial1.write(b);
}

extern "C" int serial_read() {
  if (Serial1.available() > 0) {
    return Serial1.read();
  } else {
    return -1;
  }
}

extern "C" int8_t debug_serial_write(uint8_t b) {
  return Serial.write(b);
}
