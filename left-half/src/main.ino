#include "kbd.h"
#include <HID.h>

//static const uint8_t _hidReportDescriptor[] PROGMEM = {
//    //  Keyboard
//    0x05, 0x01,                      /* USAGE_PAGE (Generic Desktop)    47 */
//    0x09, 0x06,                      /* USAGE (Keyboard) */
//    0xa1, 0x01,                      /* COLLECTION (Application) */
//    0x85, 2,   /*   REPORT_ID TODO order important? */
//    0x05, 0x07,                      /*   USAGE_PAGE (Keyboard) */
//
//    /* Keyboard Modifiers (shift, alt, ...) */
//    0x19, 0xe0,                      /*   USAGE_MINIMUM (Keyboard LeftControl) */
//    0x29, 0xe7,                      /*   USAGE_MAXIMUM (Keyboard Right GUI) */
//    0x15, 0x00,                      /*   LOGICAL_MINIMUM (0) */
//    0x25, 0x01,                      /*   LOGICAL_MAXIMUM (1) */
//    0x75, 0x01,                      /*   REPORT_SIZE (1) */
//  0x95, 0x08,                      /*   REPORT_COUNT (8) */
//    0x81, 0x02,                      /*   INPUT (Data,Var,Abs) */
//
//    /* Reserved byte, used for consumer reports, only works with linux */
//  0x05, 0x0C,                  /*   Usage Page (Consumer) */
//    0x95, 0x01,                      /*   REPORT_COUNT (1) */
//    0x75, 0x08,                      /*   REPORT_SIZE (8) */
//    0x15, 0x00,                      /*   LOGICAL_MINIMUM (0) */
//    0x26, 0xFF, 0x00,                /*   LOGICAL_MAXIMUM (255) */
//    0x19, 0x00,                      /*   USAGE_MINIMUM (0) */
//    0x29, 0xFF,                      /*   USAGE_MAXIMUM (255) */
//    0x81, 0x00,                      /*   INPUT (Data,Ary,Abs) */
//
//    /* 6 Keyboard keys */
//    0x05, 0x07,                      /*   USAGE_PAGE (Keyboard) */
//    0x95, 0x06,                      /*   REPORT_COUNT (6) */
//    0x75, 0x08,                      /*   REPORT_SIZE (8) */
//    0x15, 0x00,                      /*   LOGICAL_MINIMUM (0) */
//    0x26, 0xE7, 0x00,                /*   LOGICAL_MAXIMUM (231) */
//    0x19, 0x00,                      /*   USAGE_MINIMUM (Reserved (no event indicated)) */
//    0x29, 0xE7,                      /*   USAGE_MAXIMUM (Keyboard Right GUI) */
//    0x81, 0x00,                      /*   INPUT (Data,Ary,Abs) */
//
//    /* End */
//    0xc0                            /* END_COLLECTION */
//};



static const uint8_t _hidReportDescriptor[] PROGMEM = {

  //  Keyboard
    0x05, 0x01,                    // USAGE_PAGE (Generic Desktop)  // 47
    0x09, 0x06,                    // USAGE (Keyboard)
    0xa1, 0x01,                    // COLLECTION (Application)
    0x85, 0x02,                    //   REPORT_ID (2)
    0x05, 0x07,                    //   USAGE_PAGE (Keyboard)
   
  0x19, 0xe0,                    //   USAGE_MINIMUM (Keyboard LeftControl)
    0x29, 0xe7,                    //   USAGE_MAXIMUM (Keyboard Right GUI)
    0x15, 0x00,                    //   LOGICAL_MINIMUM (0)
    0x25, 0x01,                    //   LOGICAL_MAXIMUM (1)
    0x75, 0x01,                    //   REPORT_SIZE (1)
    
  0x95, 0x08,                    //   REPORT_COUNT (8)
    0x81, 0x02,                    //   INPUT (Data,Var,Abs)
    0x95, 0x01,                    //   REPORT_COUNT (1)
    0x75, 0x08,                    //   REPORT_SIZE (8)
    0x81, 0x03,                    //   INPUT (Cnst,Var,Abs)
    
  0x95, 0x06,                    //   REPORT_COUNT (6)
    0x75, 0x08,                    //   REPORT_SIZE (8)
    0x15, 0x00,                    //   LOGICAL_MINIMUM (0)
    0x25, 0x65,                    //   LOGICAL_MAXIMUM (101)
    0x05, 0x07,                    //   USAGE_PAGE (Keyboard)
    
  0x19, 0x00,                    //   USAGE_MINIMUM (Reserved (no event indicated))
    0x29, 0x65,                    //   USAGE_MAXIMUM (Keyboard Application)
    0x81, 0x00,                    //   INPUT (Data,Ary,Abs)
    0xc0,                          // END_COLLECTION
};

static const uint8_t _hidMultiReportDescriptorConsumer[] PROGMEM = {
  /* Consumer Control (Sound/Media keys) */
  0x05, 0x0C,                 /* usage page (consumer device) */
  0x09, 0x01,                 /* usage -- consumer control */
  0xA1, 0x01,                 /* collection (application) */
  0x85, 4,     /* report id */
  /* 4 Media Keys */
  0x15, 0x00,                 /* logical minimum */
  0x26, 0xFF, 0x03,               /* logical maximum (3ff) */
  0x19, 0x00,                 /* usage minimum (0) */
  0x2A, 0xFF, 0x03,               /* usage maximum (3ff) */
  0x95, 0x04,                 /* report count (4) */
  0x75, 0x10,                 /* report size (16) */
  0x81, 0x00,                 /* input */
  0xC0 /* end collection */
};

void setup() {
  pinMode(26, OUTPUT);
  digitalWrite(26, HIGH);
  delay(100);
  digitalWrite(26, LOW);
  delay(100);

  static HIDSubDescriptor node(_hidReportDescriptor, sizeof(_hidReportDescriptor));
  HID().AppendDescriptor(&node);

  static HIDSubDescriptor consumer_node(_hidMultiReportDescriptorConsumer, sizeof(_hidMultiReportDescriptorConsumer));
  HID().AppendDescriptor(&consumer_node);

  Serial.begin(9600);
  Serial1.begin(9600);
  //while (!Serial) ;

  digitalWrite(26, HIGH);
  delay(100);
  digitalWrite(26, LOW);
  delay(100);
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

//typedef struct
//{
//  uint8_t modifiers;
//  uint8_t reserved;
//  uint8_t keys[6];
//} KeyReport;

//extern "C" void send_key_report(KeyReport keys) {
//  HID().SendReport(2, (void*)(&keys), sizeof(KeyReport));
//}

extern "C" void hid_send_report(uint8_t report_id, const void* data, int len) {
  HID().SendReport(report_id, data, len);
}
