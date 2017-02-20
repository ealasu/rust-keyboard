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
