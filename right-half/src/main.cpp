#include <Energia.h>
#include <Wire.h>

byte val = 0;

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
}

void loop_real() {
  Wire.beginTransmission(44);
  Wire.write(val);
  Wire.endTransmission();
  val += 1;
  delay(500);
}
