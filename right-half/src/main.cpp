#include <Energia.h>
#include <Wire.h>

byte val = 0;

static int ROW_PINS[] = {
  1, 2, 3
};
#define ROW_PINS_LEN (sizeof(row_pins) / sizeof(row_pins[0]))

static int COL_PINS[] = {
  1, 2, 3
};
#define COL_PINS_LEN (sizeof(col_pins) / sizeof(col_pins[0]))

void setup()
{
  pinMode(RED_LED, OUTPUT);
}

void loop()
{
  // turn the LED on (HIGH is the voltage level)
  digitalWrite(RED_LED, HIGH);

  // wait for a second
  delay(1000);

  // turn the LED off by making the voltage LOW
  digitalWrite(RED_LED, LOW);

   // wait for a second
  delay(1000);
}

void setup_real() {
  Wire.begin();

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
}

void loop_real() {
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
    digitalWrite(row_pin, HIGH);
  }

  Wire.beginTransmission(44);
  Wire.write(&pressed_keys, 4);
  //Wire.write(val);
  //val += 1;
  Wire.endTransmission();
  delay(500);
}
