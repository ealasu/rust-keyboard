#include <Energia.h>
#include <Wire.h>

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

/*
void setup_blink()
{
  pinMode(RED_LED, OUTPUT);
}

void loop_blink()
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
*/


void setup() {
  //Wire.setModule(0);
  Wire.begin();

  Serial.begin(9600);
  Serial.println("Start!");

  //pinMode(RED_LED, OUTPUT);

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
  //digitalWrite(RED_LED, LOW);
  
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

  Serial.println("begin...");

  Wire.beginTransmission(0x66);
  Serial.println("writing...");
  //for (int i = 0; i < 10*1024; i++) {
    //for (int j = 0; i < 10; j++) {
      //Wire.write((uint8_t*) &pressed_keys, 4);
    //}
  //}
  Wire.write("hi");
  Serial.println("done writing...");
    //Wire.write(val);
    //val += 1;
  Wire.endTransmission();
  Serial.println("ended.");

  delay(100);
}
