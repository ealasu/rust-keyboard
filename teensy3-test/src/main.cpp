/*
 * Blink
 * Turns on an LED on for one second,
 * then off for one second, repeatedly.
 */

#include "Arduino.h"
//#include "Wire.h"
#include <i2c_t3.h>

#ifndef LED_BUILTIN
#define LED_BUILTIN 13
#endif


void receiveEvent(size_t count);
void requestEvent(void);

void setup()
{
  Wire.begin(I2C_SLAVE, 0x66, I2C_PINS_16_17, I2C_PULLUP_EXT, 400000);
  Wire.onReceive(receiveEvent);
  Wire.onRequest(requestEvent);
  //Wire.begin(8);
  //Wire.onReceive(receiveEvent);

  // initialize LED digital pin as an output.
  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, HIGH);
  delay(500);
  digitalWrite(LED_BUILTIN, LOW);
}

void loop()
{
  // turn the LED on (HIGH is the voltage level)
  //digitalWrite(LED_BUILTIN, HIGH);
  // wait for a second
  //delay(1000);
  // turn the LED off by making the voltage LOW
  //digitalWrite(LED_BUILTIN, LOW);
   // wait for a second
  delay(100);
}

//void receiveEvent(int howMany) {
  //digitalWrite(LED_BUILTIN, HIGH);
  //delay(1000);
  //digitalWrite(LED_BUILTIN, LOW);
  //delay(1000);
  //while (Wire.available() > 0) {
    //Wire.read();
  //}
//}

void receiveEvent(size_t count)
{

  digitalWrite(LED_BUILTIN, HIGH);
  delay(1000);
  digitalWrite(LED_BUILTIN, LOW);
  delay(1000);

    size_t idx=0;
    
    while(idx < count)
    {
        //if(idx < MEM_LEN)                     // drop data beyond mem boundary
            //databuf[idx++] = Wire.readByte(); // copy data to mem
        //else
            Wire.readByte();                  // drop data if mem full
    }
    
    //received = count; // set received flag to count, this triggers print in main loop
}

//
// handle Tx Event (outgoing I2C data)
//
void requestEvent(void)
{
    //Wire.write(databuf, MEM_LEN); // fill Tx buffer (send full mem)
}
