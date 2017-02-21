#include "kbd.h"

void setup() {
  //pinMode(LED_BUILTIN, OUTPUT);
  //digitalWrite(LED_BUILTIN, HIGH);
  //delay(200);
  //digitalWrite(LED_BUILTIN, LOW);
  //delay(200);
}

void loop() {
  //digitalWrite(LED_BUILTIN, HIGH);
  //delay(500);
  //digitalWrite(LED_BUILTIN, LOW);
  //delay(500);

  kbd_run_loop();
}

extern "C" int8_t serial_write(uint8_t b) {
  //if (Serial.availableWrite() <= 0) {
    //return 0;
  //}
  return Serial1.write(b);
  //return 1;
}

extern "C" int serial_read() {
  return Serial1.read();
}
