#include <Energia.h>
#include <HardwareSerial.h>
//#include <Wire.h>
#include "kbd.h"

byte val = 0;

static int ROW_PINS[] = {
  P1_0,
  //P1_1,
  //P1_2
};
#define ROW_PINS_LEN (sizeof(ROW_PINS) / sizeof(ROW_PINS[0]))

static int COL_PINS[] = {
  P1_4,
  //P2_1,
  //P2_2
};
#define COL_PINS_LEN (sizeof(COL_PINS) / sizeof(COL_PINS[0]))

void setup() {
  Serial.begin(9600);

  /*
  // initialize pins
  for (int i = 0; i < COL_PINS_LEN; i++) {
    int col_pin = COL_PINS[i];
    pinMode(col_pin, INPUT);
    digitalWrite(col_pin, HIGH); // pull-up TODO: make sure this works
  }
  for (int i = 0; i < ROW_PINS_LEN; i++) {
    int row_pin = ROW_PINS[i];
    pinMode(row_pin, OUTPUT);
    digitalWrite(row_pin, HIGH);
  }
  */
}

void loop() {
  //kbd_run_loop();
  
  /*
  unsigned long pressed_keys = 0; // u32
  int key_idx = 0;
  for (int row_idx = 0; row_idx < ROW_PINS_LEN; row_idx++) {
    int row_pin = ROW_PINS[row_idx];
    digitalWrite(row_pin, LOW);
    for (int col_idx = 0; col_idx < COL_PINS_LEN; col_idx++) {
      int col_pin = COL_PINS[col_idx];
      pressed_keys |= (unsigned long)(!digitalRead(col_pin) & 1) << key_idx;
      key_idx++;
    }
    delay(100);
    digitalWrite(row_pin, HIGH);
    delay(100);
  }
  */
  //Serial.write(59);
  Serial.write(65);
  Serial.write(65);
  Serial.print("Hi");
  delay(50);
}

extern "C" int8_t serial_write(uint8_t b) {
  return Serial.write(b);
}

extern "C" int serial_read() {
  return Serial.read();
}
