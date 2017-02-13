use std::env;

extern crate gcc;

fn main() {
    //let outdir = env::var("OUT_DIR").unwrap();
    //println!("cargo:rustc-link-search=native={}", outdir);
    env::set_var("CC", "arm-none-eabi-gcc");
    env::set_var("AR", "arm-none-eabi-ar");
    env::set_var("CFLAGS", "-Wall -g -Os -mcpu=cortex-m4 -mthumb -MMD -DNO_ARDUINO=1 -DF_CPU=48000000 -DUSB_SERIAL -DLAYOUT_US_ENGLISH -DUSING_MAKEFILE -DCORE_TEENSY -D__MK20DX256__ -DARDUINO=10600 -DTEENSYDUINO=121 -DUSB_KEYBOARDONLY  -I. -Iteensy3-core");
    // -DKEYMEDIA_INTERFACE
    gcc::compile_library("libteensy.a", &[
        "teensy3-core/keylayouts.c",
        "teensy3-core/usb_desc.c",
        "teensy3-core/usb_dev.c",
        "teensy3-core/usb_keyboard.c",
        "teensy3-core/usb_mem.c",
        "teensy3-core/nonstd.c",
    ]);
}
